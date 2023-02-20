from typing import TypeAlias
from common.types import HashString

from odmantic import Model
from pydantic import BaseModel

from common.types import HashString

class UserDetailsStorable(Model):
    """
    Storing new ueser's credentials.
    """

    email_address: str
    full_name: str;
    phone_number: str
    password_hash_string: HashString;

    class Config:
        collection = 'user'

class UserDisplay(BaseModel):
    """
    Return User credentials when user is created or opened.
    """

    user_id: str
    email_address: str
    owner_name: str
    phone_number: str

