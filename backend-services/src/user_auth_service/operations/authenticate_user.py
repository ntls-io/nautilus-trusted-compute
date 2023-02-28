from fastapi import HTTPException
from common.types import Engine
from .create_new_user import argon2_context
from user_auth_service.schema.actions import AuthenticateUser, AuthenticateUserResult, AuthenticateUserFailure, AuthenticateUserSuccess
from user_auth_service.schema.entities import UserDetailsStorable, UserDisplay

def verify_password(password_attempt: str, hashed_password: str) -> bool:
    return argon2_context.verify(password_attempt, hashed_password)

async def authenticate_user(engine: Engine, params: AuthenticateUser) -> AuthenticateUserResult:
    """
    Look through DB to see if any user matches to the above.
    If user exists, verify password.
    """
    existing_user = await engine.find_one(UserDetailsStorable, UserDetailsStorable.email_address == params.email_address)
    if existing_user is None:
        raise HTTPException(status_code=404, detail="This email address does not exist.")
    
    if not verify_password(params.password, existing_user.password_hash_string):
        raise HTTPException(status_code=401, detail="Incorrect Password.")

    user_display = UserDisplay(
        user_id=str(existing_user.id),
        email_address=existing_user.email_address,
        owner_name=existing_user.full_name,
        phone_number=existing_user.phone_number
    )
    return AuthenticateUserSuccess(Opened=user_display)
