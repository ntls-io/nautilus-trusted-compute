from pydantic import HttpUrl, MongoDsn

from common.settings import Settings


class AppSettings(Settings):
    """
    Application configuration settings
    """

    primary_origin: HttpUrl
    staging_mode: bool = False
    vault_db_name: str
    vault_db_connection_string: MongoDsn
