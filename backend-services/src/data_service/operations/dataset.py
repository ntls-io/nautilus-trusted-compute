from fastapi import HTTPException
from odmantic import ObjectId

from common.types import WalletAddress
from data_service.schema.actions import CreateDataset, DeleteDataset
from data_service.schema.entities import Dataset, DatasetList
from data_service.schema.types import Engine


async def create_dataset(engine: Engine, params: CreateDataset) -> Dataset:
    """
    Create a new dataset.
    """
    new_dataset = Dataset(
        wallet_id=params.wallet_id,
        data_pool_id=params.data_pool_id,
        data_schema_id=params.data_schema_id,
        name=params.name,
        description=params.description,
        num_of_rows=params.num_of_rows,
        data_pool_position=params.data_pool_position,
        created=params.created,
    )
    await engine.save(new_dataset)
    return new_dataset


async def delete_dataset(engine: Engine, params: DeleteDataset) -> None:
    """
    Delete a specified dataset.
    """
    # XXX: assumes `params.id` is a 24 character hex string
    id_to_delete = ObjectId(params.delete_id)
    existing_dataset = await engine.find_one(Dataset, Dataset.id == id_to_delete)
    if existing_dataset is None:
        raise HTTPException(404)
    await engine.delete(existing_dataset)


async def datasets(engine: Engine, wallet_id: WalletAddress) -> DatasetList:
    """
    Retrieve a list of all datasets for a given user from the database.
    """
    return await engine.find(Dataset, Dataset.wallet_id == wallet_id)


async def find_by_pool(engine: Engine, data_pool_id: str) -> DatasetList:
    """
    Retrieve a list of all datasets for a given user from the database.
    """
    return await engine.find(Dataset, Dataset.data_pool_id == data_pool_id)
