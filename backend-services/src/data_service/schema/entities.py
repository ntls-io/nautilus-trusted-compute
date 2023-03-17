from datetime import datetime
from typing import TypeAlias

from odmantic import Model

from common.types import WalletAddress


class Dataset(Model):
    """
    An address on the ledger dataseted by the user.
    """

    wallet_id: WalletAddress
    data_pool_id: str
    data_schema_id: str
    name: str
    description: str
    num_of_rows: int
    data_pool_position: int
    created: datetime


DatasetList: TypeAlias = list[Dataset]


class Datapool(Model):
    """
    An address on the ledger dataseted by the user.
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


DatapoolList: TypeAlias = list[Datapool]


class Dataschema(Model):
    """
    A JSON schema for a dataset of datapool.
    """

    name: str
    data_schema: str
    created: datetime


DataschemaList: TypeAlias = list[Dataschema]
