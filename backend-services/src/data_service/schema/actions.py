from pydantic import BaseModel, validator

from common.types import WalletAddress
from datetime import datetime

class CreateDataset(BaseModel):
    """
    Datset creation parameters.
    """

    wallet_id: WalletAddress
    data_pool_id: str
    name: str
    description: str
    length: int
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
    length: int
    datapool_hash: str
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
    length: int
    data_schema: str
    created: datetime
