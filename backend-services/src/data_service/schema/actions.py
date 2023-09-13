from pydantic import BaseModel, validator

from common.types import WalletAddress


class CreateDatapool(BaseModel):
    """
    Datapool creation parameters.
    """

    application_id: str
    creator_wallet_id: WalletAddress
    name: str
    description: str
    datapool_schema: str
    sealed_data: str
    ref_drt_id: list[str]
    contribution_token_id: str
    append_token_id: str
    ref_contributors: list[str]


class UpdateDataPool(BaseModel):
    """
    Datapool update parameters
    """

    application_id: str
    sealed_data: str
    contribution_token_id: str
    ref_contributors: str


class DeleteDatapool(BaseModel):
    """
    Datapool deletion parameters.
    """

    delete_id: str

    @validator("delete_id")
    @classmethod
    def valid_object_id_hex_representation(cls: type, v: str) -> str:
        int(v, 16)
        if len(v) != 24:
            raise AssertionError(
                f"expected a 24 character hexadecimal string but '{v}' has length {len(v)}"
            )
        return v


class CreateWasmBinary(BaseModel):
    name: str
    wasm_binary: str


class DeleteWasmBinary(BaseModel):
    delete_id: str

    @validator("delete_id")
    @classmethod
    def valid_object_id_hex_representation(cls: type, v: str) -> str:
        int(v, 16)
        if len(v) != 24:
            raise AssertionError(
                f"expected a 24 character hexadecimal string but '{v}' has length {len(v)}"
            )
        return v
