from user_auth_service.schema.actions import CreateNewUser, CreateNewUserResult, CreateNewUserSuccess, CreateNewUserFailure
from user_auth_service.schema.entities import UserDetailsStorable, UserDisplay
from data_service.schema.types import Engine
from passlib.context import CryptContext
from odmantic.bson import ObjectId

argon2_context = CryptContext(schemes=['argon2'], deprecated='auto')

def password_hash(password: str):
    return argon2_context.hash(password)


async def create_new_user(engine: Engine, params: CreateNewUser) -> CreateNewUserResult:
    """
    User Creation.
    """
    hash_password = password_hash(params.password)

    new_user = UserDetailsStorable(
        email_address=params.email_address,
        full_name=params.full_name,
        phone_number=params.phone_number,
        password_hash_string=hash_password
    )
    await engine.save(new_user)
    user_display = UserDisplay(
        user_id=new_user.id,
        email_address=new_user.email_address,
        owner_name=new_user.full_name,
        phone_number=new_user.phone_number
    )
    return CreateNewUserSuccess(Created=user_display)
