#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate sgx_tstd as std;

mod ecall_helpers;
pub mod ecalls;
pub mod ported;
pub mod schema;
pub mod vault_operations;
