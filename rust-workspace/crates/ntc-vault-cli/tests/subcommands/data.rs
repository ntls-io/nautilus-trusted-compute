//! Test the `data` subcommand

use crate::common::cli_fixture::CliFixture;

#[test]
fn usage() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["data"]).unwrap();
        let stderr = result.expect_usage_error().unwrap();
        k9::snapshot!(
            stderr,
            "
ntc-vault-data 
Manage data packages

USAGE:
    ntc-vault data <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    create     Create a new data package
    inspect    Inspect a data package

"
        );
    });
}

#[test]
fn create_usage() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["data", "create"]).unwrap();
        let stderr = result.expect_usage_error().unwrap();
        k9::snapshot!(
            stderr,
            "
error: The following required arguments were not provided:
    --metadata <METADATA>
    --schema <SCHEMA>
    --data <DATA>
    --output <OUTPUT>

USAGE:
    ntc-vault data create --metadata <METADATA> --schema <SCHEMA> --data <DATA> --output <OUTPUT>

For more information try --help

"
        );
    });
}

#[test]
fn create_no_extension() {
    CliFixture::with(|fixture| {
        let result = fixture
            .invoke([
                "data", "create", "-m", "spam", "-s", "spam", "-d", "spam", "-o", "out",
            ])
            .unwrap();
        let stderr = result.expect_app_error().unwrap();
        k9::snapshot!(
            stderr,
            r#"
Error: failed to read metadata from "spam"

Caused by:
    file has no extension: spam

"#
        );
    });
}

#[test]
fn create_bad_extension() {
    CliFixture::with(|fixture| {
        let result = fixture
            .invoke([
                "data", "create", "-m", "ham.spam", "-s", "ham.spam", "-d", "ham.spam", "-o", "out",
            ])
            .unwrap();
        let stderr = result.expect_app_error().unwrap();
        k9::snapshot!(
            stderr,
            r#"
Error: failed to read metadata from "ham.spam"

Caused by:
    unsupported file extension "spam" (ham.spam)

"#
        );
    });
}

#[test]
fn inspect_usage() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["data", "inspect"]).unwrap();
        let stderr = result.expect_usage_error().unwrap();
        k9::snapshot!(
            stderr,
            "
error: The following required arguments were not provided:
    --file <FILE>

USAGE:
    ntc-vault data inspect --file <FILE>

For more information try --help

"
        );
    });
}

#[test]
fn inspect_help() {
    CliFixture::with(|fixture| {
        let result = fixture.invoke(["data", "inspect", "-h"]).unwrap();
        let stderr = result.expect_success().unwrap();
        k9::snapshot!(
            stderr,
            "
ntc-vault-data-inspect 
Inspect a data package

USAGE:
    ntc-vault data inspect --file <FILE>

OPTIONS:
    -f, --file <FILE>    
    -h, --help           Print help information

"
        );
    });
}

#[test]
fn inspect_nonexistent() {
    CliFixture::with(|fixture| {
        let result = fixture
            .invoke(["data", "inspect", "-f", "nonexistent"])
            .unwrap();
        let stderr = result.expect_app_error().unwrap();
        k9::snapshot!(
            stderr,
            "
Error: failed to inspect file: nonexistent

Caused by:
    No such file or directory (os error 2)

"
        );
    });
}
