"""
Entry point for the Nautilus Wallet web server.
"""

from fastapi import FastAPI, status
from fastapi.middleware.cors import CORSMiddleware
from motor.motor_asyncio import AsyncIOMotorClient
from odmantic import AIOEngine

from common.types import WalletAddress
from data_service.operations.datapool import create_datapool, datapools, delete_datapool
from data_service.operations.dataschema import create_dataschema
from data_service.operations.dataset import create_dataset, datasets
from data_service.operations.dataset import delete_dataset as data_delete_dataset
from data_service.schema.actions import (
    CreateDatapool,
    CreateDataschema,
    CreateDataset,
    DeleteDatapool,
    DeleteDataset,
)
from data_service.schema.entities import (
    Datapool,
    DatapoolList,
    Dataschema,
    Dataset,
    DatasetList,
)
from user_auth_service.operations.authenticate_user import authenticate_user
from user_auth_service.operations.create_new_user import create_new_user
from user_auth_service.schema.actions import (
    AuthenticateUser,
    AuthenticateUserResult,
    CreateNewUser,
    CreateNewUserResult,
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


@app.post(
    "/auth/create", response_model=CreateNewUserResult, status_code=status.HTTP_201_CREATED
)
async def post_create_new_user(request: CreateNewUser) -> CreateNewUserResult:
    return await create_new_user(mongo_engine, request)


@app.post(
    "/auth/login", response_model=AuthenticateUserResult, status_code=status.HTTP_200_OK
)
async def post_authenticate_user(request: AuthenticateUser) -> AuthenticateUserResult:
    return await authenticate_user(mongo_engine, request)


@app.get("/datasets", response_model=DatasetList, status_code=status.HTTP_200_OK)
async def get_datasets(wallet_id: WalletAddress) -> DatasetList:
    return await datasets(mongo_engine, wallet_id)


@app.post(
    "/dataset/create", response_model=Dataset, status_code=status.HTTP_201_CREATED
)
async def post_dataset_create(request: CreateDataset) -> Dataset:
    return await create_dataset(mongo_engine, request)


@app.delete(
    "/dataset",
    response_model=None,
    status_code=status.HTTP_204_NO_CONTENT,
)
async def delete_dataset(request: DeleteDataset) -> None:
    await data_delete_dataset(mongo_engine, request)


@app.get("/datapools", response_model=DatapoolList, status_code=status.HTTP_200_OK)
async def get_datapools(wallet_id: WalletAddress) -> DatapoolList:
    return await datapools(mongo_engine, wallet_id)


@app.post(
    "/datapool/create", response_model=Datapool, status_code=status.HTTP_201_CREATED
)
async def post_datapool_create(request: CreateDatapool) -> Datapool:
    return await create_datapool(mongo_engine, request)


@app.delete(
    "/datapool",
    response_model=None,
    status_code=status.HTTP_204_NO_CONTENT,
)
async def post_delete_datapool(request: DeleteDatapool) -> None:
    await delete_datapool(mongo_engine, request)


@app.post(
    "/dataschema/create", response_model=Dataschema, status_code=status.HTTP_201_CREATED
)
async def post_dataschema_create(request: CreateDataschema) -> Dataschema:
    return await create_dataschema(mongo_engine, request)
