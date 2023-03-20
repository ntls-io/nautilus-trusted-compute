//! Supporting data types.

use std::boxed::Box;
use std::prelude::v1::String;

pub type Bytes = Box<[u8]>;

/// Nautilus Vault ID.
pub type VaultId = String;

/// A vault user's authenticating password.
pub type VaultPassword = String;

/// Algorand account seed, as bytes.
pub type AlgorandAccountSeedBytes = [u8; 32];

/// Algorand account address, as bytes.
pub type AlgorandAddressBytes = [u8; 32];

/// Algorand account address, as base32 with checksum.
pub type AlgorandAddressBase32 = String;
