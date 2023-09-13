from typing import TypeAlias

from odmantic import Model

from common.types import WalletAddress


class Datapool(Model):
    """
    An address on the ledger dataseted by the user.
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


DatapoolList: TypeAlias = list[Datapool]


class Drt(Model):
    asset_id: str
    name: str
    description: str
    url_binary: str
    price: float
    amount_created: int


DrtList: TypeAlias = list[Drt]


class WasmBinary(Model):
    name: str
    wasm_binary: str


WasmBinaryList: TypeAlias = list[WasmBinary]
