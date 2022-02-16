//! Main CLI entrypoint and subcommand dispatching.

use clap::Parser;

use crate::args::{DataCommand, IdentityCommand, VaultCommand, VaultInvocation};

pub fn main() {
    dispatch(VaultInvocation::parse())
}

/// Dispatch the given Vault CLI invocation.
fn dispatch(invocation: VaultInvocation) {
    match invocation.vault_command {
        VaultCommand::Identity { identity_command } => match identity_command {
            IdentityCommand::Create => {
                todo!()
            }
            IdentityCommand::Import => {
                todo!()
            }
            IdentityCommand::List => {
                todo!()
            }
            IdentityCommand::Destroy => {
                todo!()
            }
        },
        VaultCommand::Data { data_command } => match data_command {
            DataCommand::Create => {
                todo!()
            }
            DataCommand::Inspect => {
                todo!()
            }
        },
    }
}
