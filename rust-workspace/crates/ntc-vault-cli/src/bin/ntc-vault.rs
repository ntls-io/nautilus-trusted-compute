//! `ntc-vault` binary entrypoint.

fn main() -> anyhow::Result<()> {
    ntc_vault_cli::commands::main()
}
