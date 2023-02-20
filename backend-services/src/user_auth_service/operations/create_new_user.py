from user_auth_service.schema.actions import CreateNewUser, CreateNewUserResult, CreateNewUserSuccess, CreateNewUserFailure
from user_auth_service.schema.entites import UserDetailsStorable, UserDisplay
from common.types import Engine
from passlib.context import CryptContext

argon2_context = CryptContext(schemes=['argon2'], depricated='auto')

def password_hash(password: str):
    return argon2_context.hash(password)


async def create_new_user(engine: Engine, params: CreateNewUser) -> CreateNewUserResult:
    """
    User Creation.
    """
    hash_password = password_hash(params.password)

    new_user = UserDetailsStorable(
        email_address = params.email_address,
        full_name = params.full_name,
        phone_number = params.phone_number,
        password_hash_string = hash_password
    )
    try:
        await engine.save(new_user)
        user_display = UserDisplay(
            user_id = UserDetailsStorable.id,
            email_address = params.email_address,
            full_name = params.full_name,
            phone_number = params.phone_number
        )
        return CreateNewUserSuccess(created=user_display)
    except:
        return CreateNewUserFailure(
            Failed = 'Unable to save user credentials.'
        )
