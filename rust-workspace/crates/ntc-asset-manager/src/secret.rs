use std::{convert::Infallible, str::FromStr};
use zeroize::Zeroize;

/// The mnemonic or seed used to authenticate an account holder
#[derive(Clone, Debug, Zeroize)]
pub enum Secret {
    Mnemonic(String),
    Seed([u8; 32]),
}

impl FromStr for Secret {
    type Err = Infallible;
    fn from_str(mnemonic: &str) -> Result<Self, Self::Err> {
        Ok(Self::Mnemonic(String::from(mnemonic)))
    }
}

impl From<[u8; 32]> for Secret {
    fn from(seed: [u8; 32]) -> Self {
        Self::Seed(seed)
    }
}
