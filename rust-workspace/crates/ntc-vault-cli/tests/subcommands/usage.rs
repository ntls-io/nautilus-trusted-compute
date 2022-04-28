//! Test the top-level CLI usage.

use crate::common::cli_fixture::CliFixture;

#[test]
fn usage_no_args() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke_without_args().unwrap();
        let stderr = result.expect_usage_error().unwrap();
        k9::snapshot!(
            stderr,
            "
ntc-vault-cli 0.1.0
Nautilus Trusted Compute Vault CLI

USAGE:
    ntc-vault <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    data        Manage data packages
    identity    Manage identities

"
        );
    });
}

#[test]
fn help() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["-h"]).unwrap();
        let stderr = result.expect_success().unwrap();
        k9::snapshot!(
            stderr,
            "
ntc-vault-cli 0.1.0
Nautilus Trusted Compute Vault CLI

USAGE:
    ntc-vault <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    data        Manage data packages
    identity    Manage identities

"
        );
    });
}
