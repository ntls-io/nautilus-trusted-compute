from fastapi import HTTPException
from odmantic import ObjectId

from common.types import WalletAddress
from data_service.schema.actions import CreateDatapool, UpdateDataPool, DeleteDatapool
from data_service.schema.entities import Datapool, DatapoolList
from data_service.schema.types import Engine


async def create_datapool(engine: Engine, params: CreateDatapool) -> None:
    """
    Create a new datapool.
    """
    new_datapool = Datapool(
        creator_wallet_id=params.creator_wallet_id,
        name=params.name,
        description=params.description,
        datapool_schema=params.datapool_schema,
        datapool_hash=params.datapool_hash,
        smart_contract_id=params.smart_contract_id,
        smart_contract_address=params.smart_contract_address,
        sealed_data=params.sealed_data,
        total_rows=params.total_rows,
        created=params.created,
    )
    await engine.save(new_datapool)


async def update_datapool(engine: Engine, params: UpdateDataPool) -> None:
    """
    Update existing datapool
    """
    existing_datapool = await engine.find_one(
        Datapool,
        (UpdateDataPool.application_id == params.application_id),
    )

    if existing_datapool is None:
        raise HTTPException(404)

    if existing_datapool:
        existing_datapool.sealed_data = (params.sealed_data,)
        existing_datapool.contribution_token_id = (params.contribution_token_id,)
        existing_datapool.ref_contributors = (params.ref_contributors,)
        await engine.save(existing_datapool)
    else:
        pass


async def delete_datapool(engine: Engine, params: DeleteDatapool) -> None:
    """
    Delete a specified datapool.
    """
    # XXX: assumes `params.id` is a 24 character hex string
    id_to_delete = ObjectId(params.delete_id)
    existing_datapool = await engine.find_one(Datapool, Datapool.id == id_to_delete)
    if existing_datapool is None:
        raise HTTPException(404)
    await engine.delete(existing_datapool)


async def datapools(engine: Engine, wallet_id: WalletAddress) -> DatapoolList:
    """
    Retrieve a list of all datapools for a given user from the database.
    """
    return await engine.find(Datapool, Datapool.wallet_id == wallet_id)
