from user_auth_service.schema.entites import UserDisplay
from pydantic import BaseModel

from common.types import WalletAddress
from datetime import datetime
from typing import TypeAlias


class CreateNewUser(BaseModel):
    """
    User creation parameters.
    """

    full_name: str
    phone_number: str
    email_address: str
    password: str

class CreateNewUserSuccess(BaseModel):
    """
    Return email address, full name, and phone number.
    """

    Created: UserDisplay

class CreateNewUserFailure(BaseModel):
    """
    Return Failure if user's credentials is not created.
    """
    Failed: str

CreateNewUserResult: TypeAlias = CreateNewUserSuccess | CreateNewUserFailure

class AuthenticateUser(BaseModel):
    """
    Authentic user parameters.
    """
    email_address: str
    password: str

class AuthenticateUserSuccess(BaseModel):
    """
    Successfully authenticated user.
    """

    Opened: str

class AuthenticateUserFailure(BaseModel):
    """
    Failed to authenticate user.
    """

    Failed: str

class AuthenticateUserResult: TypeAlias = AuthenticateUserSuccess | AuthenticateUserFailure