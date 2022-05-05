use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    rand_core::{CryptoRng, RngCore},
    HpkeError, Kem as KemTrait, OpModeR, OpModeS,
};
type Kem = X25519HkdfSha256;
type Kdf = HkdfSha384;
type Aead = ChaCha20Poly1305;

// TODO: Define info that can be sent alongside the message - application defined
// like which sk to use etc.
const INFO_STR: &[u8] = b"test";

pub struct SealedMessage<'a> {
    pub encapped_key: <Kem as KemTrait>::EncappedKey,
    pub ciphertext: &'a [u8],
    pub tag: AeadTag<Aead>,
}

pub fn seal_message_in_place<'a, R: CryptoRng + RngCore>(
    msg: &'a mut [u8],
    associated_data: &[u8],
    pk_recipient: <Kem as KemTrait>::PublicKey,
    pk_sender: <Kem as KemTrait>::PublicKey,
    sk_sender: <Kem as KemTrait>::PrivateKey,
    csprng: &mut R,
) -> Result<SealedMessage<'a>, HpkeError> {
    let (encapsulated_key, mut encryption_context) = hpke::setup_sender::<Aead, Kdf, Kem, _>(
        &OpModeS::Auth((sk_sender, pk_sender)),
        &pk_recipient,
        INFO_STR,
        csprng,
    )?;

    let tag = encryption_context.seal_in_place_detached(msg, associated_data)?;

    Ok(SealedMessage {
        encapped_key: encapsulated_key,
        // TODO: don't copy
        ciphertext: msg,
        tag,
    })
}

pub fn unseal_message_in_place<'a>(
    ciphertext: &'a mut [u8],
    encapped_key: <Kem as KemTrait>::EncappedKey,
    tag: AeadTag<Aead>,
    associated_data: &[u8],
    sk_recipient: <Kem as KemTrait>::PrivateKey,
    pk_sender: <Kem as KemTrait>::PublicKey,
) -> Result<&'a [u8], HpkeError> {
    let mut receiver_ctx = hpke::setup_receiver::<Aead, Kdf, Kem>(
        &OpModeR::Auth(pk_sender),
        &sk_recipient,
        &encapped_key,
        INFO_STR,
    )?;

    receiver_ctx.open_in_place_detached(ciphertext, associated_data, &tag)?;

    // Rename for clarity
    let plaintext = ciphertext;

    Ok(plaintext)
}

#[cfg(test)]
mod test {

    use std::vec;

    use super::*;
    use rand::{self, SeedableRng};

    #[test]
    fn seal_unseal_roundtrip_success() {
        let message = vec![1, 2, 3, 4];
        let mut message_copy = message.clone();
        let associated_data = b"some associated data to share";
        let (sk_sender, pk_sender) = X25519HkdfSha256::derive_keypair(b"some key material");
        let (sk_recipient, pk_recipient) =
            X25519HkdfSha256::derive_keypair(b"some different key material");
        let mut csprng = rand::rngs::StdRng::from_entropy();

        let sealed_message = seal_message_in_place(
            &mut message_copy,
            associated_data,
            pk_recipient,
            pk_sender.clone(),
            sk_sender,
            &mut csprng,
        )
        .unwrap();

        let mut ciphertext = sealed_message.ciphertext.to_vec();

        let unsealed_message = unseal_message_in_place(
            &mut ciphertext,
            sealed_message.encapped_key,
            sealed_message.tag,
            associated_data,
            sk_recipient,
            pk_sender,
        )
        .unwrap();

        assert_eq!(unsealed_message, message);
    }
}
