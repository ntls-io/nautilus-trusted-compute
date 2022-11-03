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
        input_string: *const u8,
        input_length: usize,
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
    let input_string = String::from("Sending this string to the enclave then printing it\n");

    let mut retval = sgx_status_t::SGX_SUCCESS;

    let result = unsafe {
        append_data(
            enclave.geteid(),
            &mut retval,
            input_string.as_ptr() as *const u8,
            input_string.len(),
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
