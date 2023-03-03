from fastapi import HTTPException
from odmantic import ObjectId

from data_service.schema.actions import CreateDataschema, DeleteDataschema
from data_service.schema.entities import Dataschema
from data_service.schema.types import Engine


async def create_dataschema(engine: Engine, params: CreateDataschema) -> Dataschema:
    """
    Create a new dataschema.
    """
    new_dataschema = Dataschema(
        name=params.name,
        data_schema=params.data_schema,
        created=params.created
    )
    await engine.save(new_dataschema)
    return new_dataschema


async def delete_dataschema(engine: Engine, params: DeleteDataschema) -> None:
    """
    Delete a specified dataschema.
    """
    # XXX: assumes `params.id` is a 24 character hex string
    id_to_delete = ObjectId(params.delete_id)
    existing_dataschema = await engine.find_one(Dataschema, Dataschema.id == id_to_delete)
    if existing_dataschema is None:
        raise HTTPException(404)
    await engine.delete(existing_dataschema)


async def find_by_pool(engine: Engine, data_schema_id: str) -> list[Dataschema]:
    """
    Retrieve a list of all dataschemas for a given user from the database.
    """
    return await engine.find(Dataschema, Dataschema.data_schema_id == data_schema_id)
