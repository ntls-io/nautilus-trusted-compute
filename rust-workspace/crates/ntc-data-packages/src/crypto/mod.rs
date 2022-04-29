use hpke::{
    aead::{AeadTag, ChaCha20Poly1305},
    kdf::HkdfSha384,
    kem::X25519HkdfSha256,
    rand_core::{CryptoRng, RngCore},
    HpkeError, Kem, OpModeR, OpModeS,
};

// TODO: Define info that can be sent alongside the message - application defined
// like which sk to use etc.
const INFO_STR: &[u8] = b"test";

pub struct SealedMessage {
    pub encapped_key: <X25519HkdfSha256 as Kem>::EncappedKey,
    pub ciphertext: Box<[u8]>,
    pub tag: AeadTag<ChaCha20Poly1305>,
}

pub fn seal_message<R: CryptoRng + RngCore>(
    msg: &[u8],
    associated_data: &[u8],
    pk_recipient: <X25519HkdfSha256 as Kem>::PublicKey,
    pk_sender: <X25519HkdfSha256 as Kem>::PublicKey,
    sk_sender: <X25519HkdfSha256 as Kem>::PrivateKey,
    csprng: &mut R,
) -> Result<SealedMessage, HpkeError> {
    let (encapsulated_key, mut encryption_context) =
        hpke::setup_sender::<ChaCha20Poly1305, HkdfSha384, X25519HkdfSha256, _>(
            &OpModeS::Auth((sk_sender, pk_sender)),
            &pk_recipient,
            INFO_STR,
            csprng,
        )?;
    let mut msg_copy = msg.to_vec();
    // TODO: add option for seal in place or use seal in place instead?
    let tag = encryption_context.seal_in_place_detached(&mut msg_copy, associated_data)?;

    Ok(SealedMessage {
        encapped_key: encapsulated_key,
        ciphertext: msg_copy.into_boxed_slice(),
        tag,
    })
}

pub fn unseal_message(
    sealed_message: &SealedMessage,
    associated_data: &[u8],
    sk_recipient: <X25519HkdfSha256 as Kem>::PrivateKey,
    pk_sender: <X25519HkdfSha256 as Kem>::PublicKey,
) -> Result<Box<[u8]>, HpkeError> {
    let mut receiver_ctx = hpke::setup_receiver::<ChaCha20Poly1305, HkdfSha384, X25519HkdfSha256>(
        &OpModeR::Auth(pk_sender),
        &sk_recipient,
        &sealed_message.encapped_key,
        INFO_STR,
    )?;

    let mut cipherext_copy = sealed_message.ciphertext.to_vec();

    receiver_ctx.open_in_place_detached(
        &mut cipherext_copy,
        associated_data,
        &sealed_message.tag,
    )?;

    // Rename for clarity
    let plaintext = cipherext_copy;

    Ok(plaintext.into_boxed_slice())
}

#[cfg(test)]
mod test {

    use super::*;
    use rand::{self, SeedableRng};

    #[test]
    fn seal_unseal_roundtrip_success() {
        let message = b"some string to send";
        let associated_data = b"some associated data to share";
        let (sk_sender, pk_sender) = X25519HkdfSha256::derive_keypair(b"some key material");
        let (sk_recipient, pk_recipient) =
            X25519HkdfSha256::derive_keypair(b"some different key material");
        let mut csprng = rand::rngs::StdRng::from_entropy();

        let sealed_message = seal_message(
            message,
            associated_data,
            pk_recipient,
            pk_sender.clone(),
            sk_sender,
            &mut csprng,
        )
        .unwrap();

        let unsealed_message =
            unseal_message(&sealed_message, associated_data, sk_recipient, pk_sender).unwrap();

        assert_eq!(unsealed_message, message.to_vec().into_boxed_slice());
    }
}
