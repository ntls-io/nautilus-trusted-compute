from typing import TypeAlias

from odmantic import Model
from pydantic import BaseModel

from common.types import WalletAddress

from datetime import date

class Dataset(Model):
    """
    An address on the ledger dataseted by the user.
    """

    wallet_id: WalletAddress
    data_pool_id: str
    name: str
    description: str
    length: int
    created: date


DatasetList: TypeAlias = list[Dataset]


class DatasetDocument(BaseModel):
    """
    Database representation of a single dataset.
    """

    wallet_id: WalletAddress
    dataset: Dataset

class Datapool(Model):
    """
    An address on the ledger dataseted by the user.
    """

    creator_wallet_id: WalletAddress
    name: str
    description: str
    length: int
    datapool_hash: str
    created: date


DatapoolList: TypeAlias = list[Datapool]


class DatapoolDocument(BaseModel):
    """
    Database representation of a single datapool.
    """

    creator_wallet_id: WalletAddress
    datapool: Datapool

class Dataschema(Model):
    """
    A JSON schema for a dataset of datapool.
    """

    name: str
    data_schema: str
    created: date
