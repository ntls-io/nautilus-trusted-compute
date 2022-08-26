use actix::{Actor, Context, Handler, Message};
use sgx_types::{sgx_report_t, sgx_target_info_t, SgxResult};

use crate::traits::VaultEnclave;

/// This actor lets [`crate::resources`] interact with the vault enclave.
pub(crate) struct VaultEnclaveActor {
    pub(crate) vault_enclave: Box<dyn VaultEnclave>,
}

impl Actor for VaultEnclaveActor {
    type Context = Context<Self>;
}

// CreateReport message:

pub(crate) struct CreateReportMessage {
    pub(crate) target_info: sgx_target_info_t,
}

impl Message for CreateReportMessage {
    type Result = SgxResult<SgxResult<(sgx_report_t, [u8; 32])>>;
}

impl Handler<CreateReportMessage> for VaultEnclaveActor {
    type Result = <CreateReportMessage as Message>::Result;

    fn handle(&mut self, msg: CreateReportMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.vault_enclave.create_report(msg.target_info)
    }
}

// VaultOperation message:

pub(crate) struct VaultOperationMessage {
    pub(crate) sealed_request_bytes: Box<[u8]>,
}

impl Message for VaultOperationMessage {
    type Result = SgxResult<SgxResult<Box<[u8]>>>;
}

impl Handler<VaultOperationMessage> for VaultEnclaveActor {
    type Result = <VaultOperationMessage as Message>::Result;

    fn handle(&mut self, msg: VaultOperationMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.vault_enclave
            .vault_operation_with_retry(&msg.sealed_request_bytes)
    }
}
