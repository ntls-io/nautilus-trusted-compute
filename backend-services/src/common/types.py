from typing import NewType, TypeAlias

from odmantic import AIOEngine

WalletAddress = NewType("WalletAddress", str)
"""
A type for vault wallet addresses.  
"""

Engine: TypeAlias = AIOEngine
"""
A database engine instance.
"""

HashString = NewType("HashString", str)
"""
HashString
"""
