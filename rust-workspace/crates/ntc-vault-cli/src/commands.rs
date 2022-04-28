//! [`clap`] command and argument definitions.
//!
//! See [`crate::actions`] for the implementations of these commands.
//!
//! Quick reference:
//!
//! * <https://github.com/clap-rs/clap/blob/master/examples/tutorial_derive/README.md>
//! * <https://github.com/clap-rs/clap/blob/master/examples/derive_ref/README.md>

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::actions;

/// Top-level entrypoint.
pub fn main() -> anyhow::Result<()> {
    VaultInvocation::parse().invoke()
}

/// Nautilus Trusted Compute Vault
#[derive(Debug, Parser)]
#[clap(version, about)]
#[clap(disable_help_subcommand = true)]
#[clap(infer_subcommands = true)]
struct VaultInvocation {
    // TODO: Allow overriding identity file with global arg or env var.
    #[clap(subcommand)]
    command: VaultCommand,
}

impl VaultInvocation {
    pub fn invoke(self) -> anyhow::Result<()> {
        self.command.invoke()
    }
}

#[derive(Debug, Subcommand)]
enum VaultCommand {
    #[clap(subcommand)]
    Identity(IdentityCommand),

    #[clap(subcommand)]
    Data(DataCommand),
}

impl VaultCommand {
    fn invoke(self) -> anyhow::Result<()> {
        match self {
            VaultCommand::Identity(command) => command.invoke(),
            VaultCommand::Data(command) => command.invoke(),
        }
    }
}

/// Manage identities
#[derive(Debug, Subcommand)]
enum IdentityCommand {
    /// Create a new identity
    Create {
        /// Public name to attach to this identity.
        #[clap(long, short)]
        name: String,
    },

    /// Show the current identity
    Show,
}

impl IdentityCommand {
    fn invoke(self) -> anyhow::Result<()> {
        match self {
            IdentityCommand::Create { name } => actions::identity_create(name),
            IdentityCommand::Show => actions::identity_show(),
        }
    }
}

/// Manage data packages
#[derive(Debug, Subcommand)]
enum DataCommand {
    /// Create a new data package
    Create {
        #[clap(long, short)]
        metadata: PathBuf,

        #[clap(long, short)]
        schema: PathBuf,

        #[clap(long, short)]
        data: PathBuf,

        #[clap(long, short)]
        output: PathBuf,
    },

    /// Inspect a data package
    Inspect {
        #[clap(long, short)]
        file: PathBuf,
    },
}

impl DataCommand {
    fn invoke(&self) -> anyhow::Result<()> {
        match self {
            DataCommand::Create {
                metadata,
                schema,
                data,
                output,
            } => actions::data_create(metadata, schema, data, output),
            DataCommand::Inspect { file } => actions::data_inspect(file),
        }
    }
}
