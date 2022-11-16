#![deny(unsafe_op_in_unsafe_fn)]
#![no_std]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;
extern crate serde_json;

use std::slice;
use std::string::String;
use std::vec::Vec;

use serde_json::Value;
use sgx_types::sgx_status_t;

/// # Safety
/// Caller needs to ensure that `pool_one` points to a valid slice of length `pool_one_len`
/// Caller needs to ensure that `pool_two` points to a valid slice of length `pool_two_len`
#[no_mangle]
pub unsafe extern "C" fn append_data(pool_one: *const u8, pool_one_len: usize, pool_two: *const u8, pool_two_len: usize) -> sgx_status_t {
    let pool_1_slice = unsafe { slice::from_raw_parts(pool_one, pool_one_len) };
    // let _ = io::stdout().write(pool_1_slice);

    let pool_2_slice = unsafe { slice::from_raw_parts(pool_two, pool_two_len) };
    // let _ = io::stdout().write(pool_2_slice);

    let json_pool_1 = std::str::from_utf8(&pool_1_slice).unwrap();
    let json_pool_2 = std::str::from_utf8(&pool_2_slice).unwrap();

    println!("Message from the enclave"); //Remove 

    let pool_1 = serde_json::from_str::<Value>(&json_pool_1).expect("JSON was not well-formatted");
    let pool_2 = serde_json::from_str::<Value>(&json_pool_2).expect("JSON was not well-formatted");

    let pool_1_length = pool_1["pool"].as_array().unwrap().len();
    let pool_2_length = pool_1["pool"].as_array().unwrap().len();

    let mut new_pool: Vec<Value> = Vec::new();
    for index in 0..pool_1_length {
        new_pool.push(pool_1["pool"][index].clone());
    }
    for index in 0..pool_2_length {
        new_pool.push(pool_2["pool"][index].clone());
    }

    let pool_string = serde_json::to_string(&new_pool).unwrap();

    let json_start = r#"{"pool":"#;
    let json_end: String = String::from("}");
    let temp_pool = [json_start, &pool_string[..]].join("");
    let pool = [temp_pool, json_end].join("");

    let updated_pool = serde_json::from_str::<Value>(&pool).expect("JSON was not well-formatted");

    // Seal updated pool and store on CosmosDB
    println!("\n\n The updated pool {:#}", updated_pool);

    sgx_status_t::SGX_SUCCESS
}
