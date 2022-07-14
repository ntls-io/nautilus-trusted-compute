//! # Overview
//!
//! A library to facilitate the management of Algorand Standard Assets.
//! Primarily this involves the signing and broadcast of transactions on the
//! Algorand network.
//!
//! # Examples
//!
//! Create a new fungible token as an Algorand Standard Asset on the blockchain.
//!
//! ```no_run
//! # use std::error::Error;
//! # use tokio::macros;
//! use std::str::FromStr;
//! use ntc_asset_manager::{
//!     algorand::{Account, Algod, AlgonautAlgod},
//!     drt::{AsaNote, DrtConfigBuilder},
//!     secret::Secret,
//! };
//!
//! # #[tokio::main(flavor="current_thread")]
//! # async fn main() -> Result<(), Box<dyn Error>> {
//! let account = Account::try_from(
//!     Secret::from_str("/*your secret mnemonic*/")?
//! )?;
//! let algod = Algod(AlgonautAlgod::new(
//!     "https://example:4001",
//!     &"a".repeat(64))?,
//! );
//! let note = AsaNote {
//!     binary: Vec::from([1u8; 32]),
//!     binary_url: String::from("https://host1.example.com"),
//!     data_package: Vec::from([2u8; 32]),
//!     data_url: String::from("https://host2.example.com"),
//! };
//!
//! let config = DrtConfigBuilder::new([3u8;32], note)
//!     .name("DRT")
//!     .supply(10)
//!     .url("https://drt.example.com")?
//!     .build();
//! config.submit(&algod, &account).await;
//! # Ok(())
//! # }
//! ```

pub mod algorand;
pub mod drt;
pub mod secret;
