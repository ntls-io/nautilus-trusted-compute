//! Fixture-based CLI tests.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::{fs, io};

use anyhow::{anyhow, Context};
use assert_cmd::Command;
use ntc_data_packages::identity::VaultIdentity;
use ntc_vault_cli::identity_files::VaultIdentityConfig;
use tempfile::TempDir;
use walkdir::{DirEntry, WalkDir};

/// CLI executable name.
const CLI_NAME: &str = "ntc-vault";

/// Config file path, relative to the home directory.
const CONFIG_REL_PATH: &str = ".config/ntc-vault/identity.toml";

/// A fixture for invoking CLI commands in.
#[derive(Debug)]
pub struct CliFixture {
    base_dir: TempDir,
}

impl CliFixture {
    pub fn with<R>(f: fn(&Self) -> R) -> R {
        let fixture = Self::new().unwrap();
        let result = f(&fixture);
        fixture.close().unwrap();
        result
    }

    pub fn new() -> anyhow::Result<Self> {
        let base = TempDir::new()?;
        let fixture = Self { base_dir: base };
        fs::create_dir(fixture.home_dir())?;
        fs::create_dir(fixture.current_dir())?;
        Ok(fixture)
    }

    fn home_dir(&self) -> PathBuf {
        self.base_dir.path().join("home")
    }

    fn current_dir(&self) -> PathBuf {
        self.base_dir.path().join("cwd")
    }

    pub fn command(&self) -> anyhow::Result<FixtureCommand> {
        FixtureCommand::new(self)
    }

    pub fn invoke_without_args(&self) -> anyhow::Result<InvocationResult> {
        let mut command = self.command()?;
        let result: InvocationResult = command.invoke()?;
        Ok(result)
    }

    pub fn invoke(
        &self,
        args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> anyhow::Result<InvocationResult> {
        self.command()?.args(args).invoke()
    }

    pub fn list_files(&self) -> anyhow::Result<Vec<PathBuf>> {
        list_files(self.base_dir.path())
    }

    pub fn close(self) -> io::Result<()> {
        self.base_dir.close()
    }
}

fn list_files(base: &Path) -> anyhow::Result<Vec<PathBuf>> {
    WalkDir::new(base)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) if entry.file_type().is_dir() => None, // Skip directories
            Ok(entry) => Some(rel_path(base, entry)),        // List files
            Err(err) => Some(Err(err.into())),               // Propagate errors
        })
        .collect()
}

fn rel_path(base: &Path, entry: DirEntry) -> anyhow::Result<PathBuf> {
    let rel_path = entry.path().strip_prefix(base)?;
    Ok(rel_path.to_path_buf())
}

/// A [`Command`] in the context of a [`CliFixture`].
pub struct FixtureCommand<'a> {
    fixture: &'a CliFixture,
    command: Command,
}

impl<'a> FixtureCommand<'a> {
    fn new(fixture: &'a CliFixture) -> anyhow::Result<Self> {
        let mut command = Command::cargo_bin(CLI_NAME)?;
        command.env_clear();
        command.env("HOME", fixture.home_dir());
        command.current_dir(&fixture.current_dir());
        Ok(Self { fixture, command })
    }
    pub fn args(mut self, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Self {
        self.command.args(args);
        self
    }

    pub fn invoke(&mut self) -> anyhow::Result<InvocationResult<'a>> {
        let output = self.command.output()?;
        let mut result = InvocationResult::from_output(self.fixture, output)?;
        result.redact_defaults()?;
        Ok(result)
    }
}

/// The result of invoking a [`FixtureCommand`].
#[derive(Debug)]
pub struct InvocationResult<'a> {
    fixture: &'a CliFixture,

    /// Like [`Output`], but assume a non-signal exit code.
    pub status: i32,

    /// Like [`Output`], but assume a UTF-8 string result.
    pub stdout: String,

    /// Like [`Output`], but assume a UTF-8 string result.
    pub stderr: String,
}

impl<'a> InvocationResult<'a> {
    /// Convert an [`Output`] to an [`InvocationResult`].
    ///
    /// Fail if the CLI binary was terminated by a signal, or if `stdout` and `stderr` are not valid UTF-8.
    pub fn from_output(fixture: &'a CliFixture, output: Output) -> anyhow::Result<Self> {
        Ok(Self {
            fixture,
            status: output.status.code().ok_or_else(|| {
                anyhow!("expected non-signal exit status, got {:?}", output.status)
            })?,
            stdout: String::from_utf8(output.stdout).context("expected UTF-8 stdout")?,
            stderr: String::from_utf8(output.stderr).context("expected UTF-8 stderr")?,
        })
    }

    /// Redact a string from `stdout` and `stderr`.
    fn redact(&mut self, from: &str, to: &str) {
        for s in [&mut self.stdout, &mut self.stderr] {
            *s = s.replace(from, to);
        }
    }

    /// Redact the home directory prefix.
    fn redact_home_dir(&mut self, to: &str) {
        let home_dir = self.fixture.home_dir();
        let home_dir_str = home_dir.to_str().unwrap();
        self.redact(home_dir_str, to)
    }

    /// Redact the configured public key, if any
    fn redact_public_key(&mut self, to: &str) -> anyhow::Result<()> {
        let config_path = self.fixture.home_dir().join(CONFIG_REL_PATH);
        if config_path.exists() {
            let config = VaultIdentityConfig::load(&config_path)?;
            let identity: VaultIdentity = config.into();
            let pk = identity.get_sign_public_key();
            let pk_base64 = base64::encode(pk.as_ref());
            self.redact(&pk_base64, to);
        }
        Ok(())
    }

    /// Redact a default set of values.
    fn redact_defaults(&mut self) -> anyhow::Result<()> {
        self.redact_home_dir("${HOME}");
        self.redact_public_key("<<PUBLIC KEY>>")?;
        Ok(())
    }

    pub fn expect_success(&self) -> anyhow::Result<&str> {
        if self.status == 0 && self.stderr.is_empty() {
            Ok(&self.stdout)
        } else {
            Err(anyhow!("{self:?}"))
        }
    }

    pub fn expect_error_code(&self, code: i32) -> anyhow::Result<&str> {
        if self.status == code && self.stdout.is_empty() {
            Ok(&self.stderr)
        } else {
            Err(anyhow!("{self:?}"))
        }
    }

    pub fn expect_app_error(&self) -> anyhow::Result<&str> {
        self.expect_error_code(1)
    }
    pub fn expect_usage_error(&self) -> anyhow::Result<&str> {
        self.expect_error_code(2)
    }
}
