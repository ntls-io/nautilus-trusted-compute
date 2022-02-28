# Nautilus Trusted Compute Architecture

This document describes the high-level architecture of Nautilus Trusted Compute.

(Inspired by Aleksey Klado's [ARCHITECTURE.md])

[ARCHITECTURE.md]: https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html

## Cargo workspaces

The repository is organised into Cargo workspaces, each containing a flat layout of crates under `crates/*`, as described in "[Large Rust Workspaces]".

[Large Rust Workspaces]: https://matklad.github.io/2021/08/22/large-rust-workspaces.html

### `rust-workspace/crates/*`

This workspace contains all crates that compile with the Rust's `stable` toolchain.

###  `rust-sgx-workspace/crates/*`

This workspace contains all "SGX-flavoured" crates: that is, all crates that depend on [incubator-teaclave-sgx-sdk] and a supported Rust `nightly` toolchain to compile.

[incubator-teaclave-sgx-sdk]: https://github.com/apache/incubator-teaclave-sgx-sdk

**Architecture Invariant:** `rust-sgx-workspace` crates may depend on `rust-workspace` crates, but not the other way around.
