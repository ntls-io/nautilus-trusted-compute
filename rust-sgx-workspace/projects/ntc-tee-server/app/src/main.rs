extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

static ENCLAVE_FILE: &str = "enclave.signed.so";

extern "C" {

    // Update function to ecall two (sealed) data pools from CosmosDB
    // Use Data API in nautilus-trusted-compute/rust-workspace/projects/data-server/src/data_handlers
    fn append_data(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        pool_one: *const u8,
        pool_one_length: usize,
        pool_two: *const u8,
        pool_two_length: usize,
    ) -> sgx_status_t;

}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        ENCLAVE_FILE,
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}

fn main() {
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    // Update - send sealed binary data into enclave (from CosmosDB)
    // Test pool 1 - Import existing data pool from CosmosDB
    let pool_one = r#"
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
    let pool_two = r#"
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

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let result = unsafe {
        append_data(
            enclave.geteid(),
            &mut retval,
            pool_one.as_ptr() as *const u8,
            pool_one.len(),
            pool_two.as_ptr() as *const u8,
            pool_two.len(),
        )
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {
            println!("[+] Ecall success...");
        }
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    enclave.destroy();
}
