/* automatically generated by rust-bindgen 0.59.2 */

use sgx_types::*;

extern "C" {
    pub fn enclave_create_report(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        p_qe3_target: *const sgx_target_info_t,
        p_report: *mut sgx_report_t,
        enclave_data: *mut [u8; 32usize],
    ) -> sgx_status_t;
}
extern "C" {
    pub fn wallet_operation(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        sealed_request_buffer: *const u8,
        sealed_request_size: size_t,
        sealed_response_buffer: *mut u8,
        sealed_response_capacity: size_t,
        sealed_response_used: *mut size_t,
    ) -> sgx_status_t;
}
