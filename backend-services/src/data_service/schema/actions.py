from datetime import datetime

from pydantic import BaseModel, validator

from common.types import WalletAddress


class CreateDataset(BaseModel):
    """
    Datset creation parameters.
    """

    wallet_id: WalletAddress
    data_pool_id: str
    data_schema_id: str
    name: str
    description: str
    num_of_rows: int
    data_pool_position: int
    created: datetime


class DeleteDataset(BaseModel):
    """
    Dataset deletion parameters.
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


class CreateDatapool(BaseModel):
    """
    Datpool creation parameters.
    """

    creator_wallet_id: WalletAddress
    name: str
    description: str
    datapool_hash: str
    smart_contract_id: str
    smart_contract_address: str
    sealed_data: str
    total_rows: int
    created: datetime


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


class CreateDataschema(BaseModel):
    """
    Schema creation parameters.
    """

    name: str
    data_schema: str
    created: datetime


class DeleteDataschema(BaseModel):
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
