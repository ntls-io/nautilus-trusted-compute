from datetime import datetime
from unittest.mock import AsyncMock

import pytest
from motor import motor_asyncio
from odmantic import AIOEngine, ObjectId
from pytest_mock import MockerFixture

from common.types import WalletAddress
from data_service.operations.dataset import create_dataset, delete_dataset
from data_service.schema.actions import CreateDataset, DeleteDataset
from data_service.schema.entities import Dataset


@pytest.mark.asyncio
async def test_create_dataset_success(mocker: MockerFixture) -> None:
    mocker.patch("motor.motor_asyncio.AsyncIOMotorClient")
    test_create_dataset = CreateDataset(
        wallet_id=WalletAddress("test_wallet_id"),
        data_pool_id="x0x0x0x0x0x0x0x0x0x0x0",
        data_schema_id="x0x0x0x0x0x0x0x0x0x0x0",
        name="test_name",
        description="test description of datasete",
        num_of_rows=500,
        data_pool_position=0,
        created=datetime.now(),
    )

    mock_save = AsyncMock(return_value=test_create_dataset)
    mocker.patch.object(AIOEngine, "save", mock_save)
    engine = AIOEngine(client=motor_asyncio.AsyncIOMotorClient())

    returned_dataset = await create_dataset(engine, test_create_dataset)
    mock_save.assert_awaited_once_with(returned_dataset)


@pytest.mark.asyncio
async def test_delete_dataset_success(mocker: MockerFixture) -> None:
    hex_string_id = "a" * 24
    dataset_to_delete = Dataset.parse_obj(
        {
            "id": ObjectId(hex_string_id),
            "wallet_id": "test_wallet_id",
            "data_pool_id": "x0x0x0x0x0x0x0x0x0x0x0",
            "data_schema_id": "x0x0x0x0x0x0x0x0x0x0x0",
            "name": "test_name1",
            "description": "test description of dataset",
            "num_of_rows": 500,
            "data_pool_position": 0,
            "created": datetime.now(),
        }
    )
    mocker.patch("motor.motor_asyncio.AsyncIOMotorClient")

    mock_find_one = AsyncMock(return_value=dataset_to_delete)
    mock_delete = AsyncMock(return_value=None)
    mocker.patch.object(AIOEngine, "find_one", mock_find_one)
    mocker.patch.object(AIOEngine, "delete", mock_delete)

    engine = AIOEngine(client=motor_asyncio.AsyncIOMotorClient())

    params = DeleteDataset(delete_id=hex_string_id)

    assert await delete_dataset(engine, params) is None
    mock_find_one.assert_awaited_once_with(
        Dataset, Dataset.id == ObjectId(params.delete_id)
    )
    mock_delete.assert_awaited_once_with(dataset_to_delete)


@pytest.mark.asyncio
async def test_get_datasets_success(mocker: MockerFixture) -> None:
    stored_docs = [
        {
            "id": ObjectId(b"a" * 12),
            "wallet_id": "test_wallet_id",
            "data_pool_id": "x0x0x0x0x0x0x0x0x0x0x0",
            "data_schema_id": "x0x0x0x0x0x0x0x0x0x0x0",
            "name": "test_name1",
            "description": "test description of dataset",
            "num_of_rows": 500,
            "data_pool_position": 0,
            "created": datetime.now(),
        },
        {
            "id": ObjectId(b"b" * 12),
            "wallet_id": "test_wallet_id2",
            "data_pool_id": "x0x0x0x0x0x0x0x0x0x0x0",
            "data_schema_id": "x0x0x0x0x0x0x0x0x0x0x0",
            "name": "test_name2",
            "description": "test description of dataset again",
            "num_of_rows": 200,
            "data_pool_position": 0,
            "created": datetime.now(),
        },
    ]
    mock_find = AsyncMock(return_value=stored_docs)
    mocker.patch("motor.motor_asyncio.AsyncIOMotorClient")
    mocker.patch.object(AIOEngine, "find", mock_find)
