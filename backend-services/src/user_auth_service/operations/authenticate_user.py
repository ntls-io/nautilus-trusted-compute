from fastapi import HTTPException

from common.types import Engine
from datetime import datetime, timedelta
from user_auth_service.schema.actions import (
    AuthenticateUser,
    AuthenticateUserResult,
    AuthenticateUserSuccess,
)
from user_auth_service.schema.entities import UserDetailsStorable, UserDisplay

from .create_new_user import argon2_context

from jose import jwt

SECRET_KEY = ""
ALGORITHM = "HS256"


def create_access_token(email_address: str,
                        user_id: str,
                        owner_name: str,
                        phone_number: str,
                        expires_delta: timedelta | None = None) -> str:
    encode = {"email": email_address, "user_id": user_id, "owner_name": owner_name, "phone_number": phone_number}
    if expires_delta:
        expire = datetime.utcnow() + expires_delta
    else:
        expire = datetime.utcnow() + timedelta(minutes=15)
    encode.update({"exp": expire})
    return jwt.encode(encode, SECRET_KEY, algorithm=ALGORITHM)

def verify_password(password_attempt: str, hashed_password: str) -> bool:
    return argon2_context.verify(password_attempt, hashed_password)

async def authenticate_user(engine: Engine, params: AuthenticateUser) -> AuthenticateUserResult:
    """
    Authenticate User
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

    token_expires = timedelta(minutes=15)
    token = create_access_token(user_display.email_address, user_display.user_id,
                                user_display.owner_name, user_display.phone_number, token_expires)
    return AuthenticateUserSuccess(Opened=token)
