use ed25519::signature::Signer;

pub(crate) struct AuthDataSigner<S>
where
    S: Signer<ed25519::Signature>,
{
    pub signing_key: S,
}

impl<S> AuthDataSigner<S>
where
    S: Signer<ed25519::Signature>,
{
    pub(crate) fn sign(&self, auth_data: &[u8]) -> ed25519::Signature {
        // NOTE: use `try_sign` if you'd like to be able to handle
        // errors from external signing services/devices (e.g. HSM/KMS)
        // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
        self.signing_key.sign(auth_data)
    }
}

#[cfg(test)]
use ed25519::signature::Verifier;

#[cfg(test)]
pub(crate) struct AuthDataVerifier<V> {
    pub verify_key: V,
}

#[cfg(test)]
impl<V> AuthDataVerifier<V>
where
    V: Verifier<ed25519::Signature>,
{
    #[cfg(test)]
    pub(crate) fn verify(
        &self,
        auth_data: &[u8],
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verify_key.verify(auth_data, signature)
    }
}
