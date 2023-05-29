# Rust SGX workspace

This workspace requires mutually-compatible versions of the following tools,
listed here with their current versions:

1. **Rust SGX SDK:** 1.1.4 + updates (revision [e8a9fc22939befa27ff67f5509b2c2dfe8499945])
2. **Rust toolchain:** `nightly-2021-11-01`
3. **Intel SGX SDK:** [2.15.1]

[e8a9fc22939befa27ff67f5509b2c2dfe8499945]: https://github.com/apache/incubator-teaclave-sgx-sdk/commit/e8a9fc22939befa27ff67f5509b2c2dfe8499945

[2.15.1]: https://01.org/intel-softwareguard-extensions/downloads/intel-sgx-linux-2.15.1-release

To install and switch between versions of the SDK packages, you can use the [rust-sgx-sdk-dev-env] helper scripts.

[rust-sgx-sdk-dev-env]: https://github.com/PiDelport/rust-sgx-sdk-dev-env

## Notes

* The Rust SGX SDK Git revision should be pinned and patched for all crates and all SGX dependencies in this project, to avoid Cargo dependency resolution and compilation issues.

* When adding or upgrading SGX dependencies, monitor `Cargo.lock` to check that the change does not introduce multiple revisions of the same underlying SGX SDK crates: these should be patched to refer to the pinned revision above.
