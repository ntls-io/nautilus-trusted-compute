"""
Entry point for Trusted Compute web server.
"""

from fastapi import FastAPI, status
from fastapi.middleware.cors import CORSMiddleware
from motor.motor_asyncio import AsyncIOMotorClient
from odmantic import AIOEngine

from common.types import WalletAddress
from data_service.operations.datapool import (
    create_datapool,
    update_datapool,
    datapools,
    delete_datapool,
)
from data_service.operations.wasm import (
    create_wasm_binary,
    get_wasm_binary,
    delete_wasm_binary,
)
from data_service.schema.actions import (
    CreateDatapool,
    UpdateDataPool,
    DeleteDatapool,
    CreateWasmBinary,
    DeleteWasmBinary,
)
from data_service.schema.entities import (
    Datapool,
    DatapoolList,
    WasmBinaryList,
)
from web_asgi.settings import AppSettings

app_settings = AppSettings()
mongo_client = AsyncIOMotorClient(app_settings.vault_db_connection_string)
mongo_engine = AIOEngine(
    client=mongo_client,
    database=app_settings.vault_db_name,
)

origins = [str(app_settings.primary_origin)]
if app_settings.staging_mode:
    origins.append("http://localhost:4200")

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["GET", "POST", "HEAD", "DELETE"],
    allow_headers=["*"],
)


@app.get("/datapools", response_model=DatapoolList, status_code=status.HTTP_200_OK)
async def get_datapools(wallet_id: WalletAddress) -> DatapoolList:
    return await datapools(mongo_engine, wallet_id)


@app.post(
    "/datapool/create", response_model=Datapool, status_code=status.HTTP_201_CREATED
)
async def post_datapool_create(request: CreateDatapool) -> Datapool:
    return await create_datapool(mongo_engine, request)


@app.post(
    "/datapool/update", response_model=Datapool, status_code=status.HTTP_201_CREATED
)
async def post_datapool_update(request: UpdateDataPool) -> Datapool:
    return await update_datapool(mongo_engine, request)


@app.delete(
    "/datapool",
    response_model=None,
    status_code=status.HTTP_204_NO_CONTENT,
)
async def post_delete_datapool(request: DeleteDatapool) -> None:
    await delete_datapool(mongo_engine, request)


@app.get("/wasm", response_model=WasmBinaryList, status_code=status.HTTP_200_OK)
async def get_wasm_binary(name: str) -> WasmBinaryList:
    return await datapools(mongo_engine, name)


@app.post("/wasm", response_model=None, status_code=status.HTTP_201_CREATED)
async def post_wasm_binary_post(request: CreateWasmBinary) -> None:
    return await create_wasm_binary(mongo_engine, request)


@app.delete(
    "/wasm",
    response_model=None,
    status_code=status.HTTP_204_NO_CONTENT,
)
async def post_delete_wasm_binary(request: DeleteWasmBinary) -> None:
    await delete_wasm_binary(mongo_engine, request)
