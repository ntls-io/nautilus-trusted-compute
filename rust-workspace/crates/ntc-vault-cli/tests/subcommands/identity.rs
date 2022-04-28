//! Test the `identity` subcommand.

use crate::common::cli_fixture::CliFixture;

#[test]
fn usage() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["identity"]).unwrap();
        let stderr = result.expect_usage_error().unwrap();
        k9::snapshot!(
            stderr,
            "
ntc-vault-identity 
Manage identities

USAGE:
    ntc-vault identity <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    create    Create a new identity
    show      Show the current identity

"
        );
    });
}

#[test]
fn create_no_args() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["identity", "create"]).unwrap();
        let stderr = result.expect_usage_error().unwrap();
        k9::snapshot!(
            stderr,
            "
error: The following required arguments were not provided:
    --name <NAME>

USAGE:
    ntc-vault identity create --name <NAME>

For more information try --help

"
        );
    });
}

#[test]
fn create_with_name() {
    CliFixture::with(|fixture| {
        let result = fixture
            .invoke(["identity", "create", "--name", "Test User"])
            .unwrap();
        let stdout = result.expect_success().unwrap();

        k9::snapshot!(
            stdout,
            "
Identity created at ${HOME}/.config/ntc-vault/identity.toml

"
        );
        let files = fixture.list_files().unwrap();
        k9::snapshot!(
            files,
            r#"
[
    "home/.config/ntc-vault/identity.toml",
]
"#
        );
    });
}

#[test]
fn show_not_configured() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["identity", "show"]).unwrap();
        let stderr = result.expect_app_error().unwrap();
        k9::snapshot!(
            stderr,
            "
Error: Identity not configured

Caused by:
    File not found: ${HOME}/.config/ntc-vault/identity.toml

"
        );
    });
}

#[test]
fn create_show() {
    CliFixture::with(|fixture| {
        fixture
            .invoke(["identity", "create", "--name", "Test User"])
            .unwrap();
        let result = fixture.invoke(["identity", "show"]).unwrap();
        let stdout = result.expect_success().unwrap();
        k9::snapshot!(
            stdout,
            "
Path:       ${HOME}/.config/ntc-vault/identity.toml
Name:       Test User
Public key: <<PUBLIC KEY>>

"
        );
    });
}
