//! CLI subcommand and argument definitions.
//!
//! Quick reference:
//!
//! * <https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/README.md>
//! * <https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md>

use clap::{Parser, Subcommand};

/// Nautilus Trusted Compute Vault
#[derive(Debug)] // core
#[derive(Parser)] // clap
#[clap(version, about)]
#[clap(disable_help_subcommand = true)]
#[clap(infer_subcommands = true)]
pub(crate) struct VaultInvocation {
    #[clap(subcommand)]
    pub(crate) vault_command: VaultCommand,
}

#[derive(Debug)] // core
#[derive(Subcommand)] // clap
pub(crate) enum VaultCommand {
    Identity {
        #[clap(subcommand)]
        identity_command: IdentityCommand,
    },
    Data {
        #[clap(subcommand)]
        data_command: DataCommand,
    },
}

/// Manage identities
#[derive(Debug)] // core
#[derive(Subcommand)] // clap
pub(crate) enum IdentityCommand {
    /// Create a new identity
    Create,
    /// Import an identity
    Import,
    /// List identities
    List,
    /// Destroy an identity
    Destroy,
}

/// Manage data packages
#[derive(Debug)] // core
#[derive(Subcommand)] // clap
pub(crate) enum DataCommand {
    /// Create a new data package
    Create,
    /// Create a new data package
    Inspect,
}
