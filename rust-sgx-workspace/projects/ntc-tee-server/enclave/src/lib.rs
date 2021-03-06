#![deny(unsafe_op_in_unsafe_fn)]
#![no_std]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;

use std::io::{self, Write};
use std::slice;

use sgx_types::sgx_status_t;

/// Does a test ecall
///
/// # Safety
/// Caller needs to ensure that `some_string` points to a valid slice of length `some_len`
#[no_mangle]
pub unsafe extern "C" fn ecall_test(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    println!("Message from the enclave");

    sgx_status_t::SGX_SUCCESS
}
