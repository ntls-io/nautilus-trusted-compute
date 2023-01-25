#![deny(unsafe_op_in_unsafe_fn)]
#![no_std]

extern crate sgx_types;
#[macro_use]
extern crate sgx_tstd as std;
extern crate serde_json;

use std::io::{self, Write};
use std::slice;
use std::string::String;
use std::vec::Vec;

use serde_json::Value;
use sgx_types::sgx_status_t;

/// # Safety
/// Caller needs to ensure that `some_string` points to a valid slice of length `some_len`
#[no_mangle]
pub unsafe extern "C" fn append_data(some_string: *const u8, some_len: usize) -> sgx_status_t {
    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    println!("Message from the enclave");

    // Test pool 1 - Import existing data pool from CosmosDB
    let json_pool_1 = r#"
    {
        "pool": [
            { 
                "ID":1,
                "Income":109748.717064175,
                "Postal":41,
                "WalletID":"9173f117-eaf5-443b-a405-c3d5e2aa5d99"
                },
                { 
                "ID":2,
                "Income":77356.0675380472,
                "Postal":15,
                "WalletID":"9f86ae7c-ac1c-4b8e-9daf-bd40ad13e157"
                },
                { 
                "ID":3,
                "Income":33746.4377853607,
                "Postal":11,
                "WalletID":"3e77ee27-b25b-4bcc-b9b8-d7107a633ae9"
                },
                { 
                "ID":4,
                "Income":37657.0518817567,
                "Postal":48,
                "WalletID":"67be8d2b-c31a-44b6-9cb0-6456072d3353"
                },
                { 
                "ID":5,
                "Income":27353.1866838295,
                "Postal":5,
                "WalletID":"412326e0-716a-4233-bae1-4cd199207893"
                }
        ]
    }
    "#;

    // Test pool 2 - Import append pool from CosmosDB (Temp data pool)
    let json_pool_2 = r#"
    {
        "pool": [
            { 
                "ID":6,
                "Income":71540.129423949,
                "Postal":34,
                "WalletID":"dabcbe30-4ed0-4c3b-b6b3-aafb85312963"
                },
                { 
                "ID":7,
                "Income":7352.38903575921,
                "Postal":41,
                "WalletID":"fdcd0caa-48bf-446f-b00f-a90d646f55c3"
                },
                { 
                "ID":8,
                "Income":110742.078725787,
                "Postal":7,
                "WalletID":"f4f5cd1b-c2ad-4587-905d-47237ebdbd89"
                },
                { 
                "ID":9,
                "Income":129564.725166652,
                "Postal":3,
                "WalletID":"7e4e80c4-7e9b-4a6e-8d94-e4f72bde8233"
                },
                { 
                "ID":10,
                "Income":2964.80996131516,
                "Postal":33,
                "WalletID":"ecefdd47-aab3-4595-87f7-cf9bbef46a53"
                }
        ]
    }
    "#;

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
