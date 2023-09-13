from fastapi import HTTPException
from odmantic import ObjectId

from data_service.schema.actions import CreateWasmBinary, DeleteWasmBinary
from data_service.schema.entities import WasmBinary, WasmBinaryList
from data_service.schema.types import Engine


async def create_wasm_binary(engine: Engine, params: CreateWasmBinary) -> None:
    """
    Store a new WASM Binary.
    """
    new_wasm_binary = WasmBinary(name=params.name, wasm_binary=params.wasm_binary)
    await engine.save(new_wasm_binary)


async def get_wasm_binary(engine: Engine, name: str) -> WasmBinaryList:
    """
    Retrieve a WASM BInary.
    """
    return await engine.find(WasmBinary, WasmBinary.name == name)


async def delete_wasm_binary(engine: Engine, params: DeleteWasmBinary) -> None:
    """
    Delete a specified WASM Binary.
    """
    # XXX: assumes `params.id` is a 24 character hex string
    id_to_delete = ObjectId(params.delete_id)
    existing_wasm_binary = await engine.find_one(
        WasmBinary, WasmBinary.id == id_to_delete
    )
    if existing_wasm_binary is None:
        raise HTTPException(404)
    await engine.delete(existing_wasm_binary)
