use ed25519::signature::{Signer, Verifier};

use crate::handlers::auth_data::AuthData;

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
    pub(crate) fn sign(&self, auth_data: &AuthData) -> ed25519::Signature {
        // NOTE: use `try_sign` if you'd like to be able to handle
        // errors from external signing services/devices (e.g. HSM/KMS)
        // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
        self.signing_key
            .sign(serde_json::to_vec(&auth_data).unwrap().as_slice())
    }
}
#[allow(dead_code)]
pub(crate) struct AuthDataVerifier<V> {
    pub verify_key: V,
}

impl<V> AuthDataVerifier<V>
where
    V: Verifier<ed25519::Signature>,
{
    #[allow(dead_code)]
    pub(crate) fn verify(
        &self,
        auth_data: &AuthData,
        signature: &ed25519::Signature,
    ) -> Result<(), ed25519::Error> {
        self.verify_key.verify(
            serde_json::to_vec(&auth_data).unwrap().as_slice(),
            signature,
        )
    }
}
