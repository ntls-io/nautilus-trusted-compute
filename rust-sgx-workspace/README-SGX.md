# Rust SGX workspace

This workspace requires mutually-compatible versions of the following tools,
listed here with their current versions:

1. **Rust SGX SDK:** 1.1.6 + updates (revision [f1776a7cec1caab2959813f87bb4924805b92011])
2. **Rust toolchain:** `nightly-2022-10-01`
3. **Intel SGX SDK:** [2.18]

[f1776a7cec1caab2959813f87bb4924805b92011]: https://github.com/apache/incubator-teaclave-sgx-sdk/commit/f1776a7cec1caab2959813f87bb4924805b92011

[2.18]: https://download.01.org/intel-sgx/sgx-linux/2.18/docs/Intel_SGX_SDK_Release_Notes_Linux_2.18_Open_Source.pdf

To install and switch between versions of the SDK packages, you can use the [rust-sgx-sdk-dev-env] helper scripts.

[rust-sgx-sdk-dev-env]: https://github.com/ntls-io/rust-sgx-sdk-dev-env

## Notes

* The Rust SGX SDK Git revision should be pinned and patched for all crates and all SGX dependencies in this project, to avoid Cargo dependency resolution and compilation issues.

* When adding or upgrading SGX dependencies, monitor `Cargo.lock` to check that the change does not introduce multiple revisions of the same underlying SGX SDK crates: these should be patched to refer to the pinned revision above.
