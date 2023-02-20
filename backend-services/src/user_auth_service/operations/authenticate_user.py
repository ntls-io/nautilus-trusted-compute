from common.types import Engine
from create_new_user import argon2_context
from fastapi import HTTPException
from user_auth_service.schema.actions import AuthenticateUser, AuthenticateUserResult, AuthenticateUserSuccess, AuthenticateUserFailure
from user_auth_service.schema.entites import UserDetailsStorable

def verify_password(password_attempt: str, hashed_password: str):
    return argon2_context.verify(password_attempt, hashed_password)

async def authenticate_user(engine: Engine, params: AuthenticateUser) -> AuthenticateUserResult:
    """
    Look through DB to see if any user matches to the above.
    If user exists, verify password.
    """
    existing_user = await engine.find_one(UserDetailsStorable, UserDetailsStorable.email_address == AuthenticateUser.email_address)
    if existing_user is None:
        return AuthenticateUserFailure(
            Failed = 'Username does not exist.' 
        )
    
    if not verify_password(AuthenticateUser.password,existing_user.hashed_password):
        return AuthenticateUserFailure(
            Failed = 'Invalid Password' 
        )
    return existing_user