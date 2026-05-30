import asyncio
from typing import Optional, Dict
import hashlib

class IPFSBackbone:
    def __init__(self):
        self.local_storage: Dict[str, bytes] = {}

    async def add_content(self, data: bytes) -> str:
        cid = hashlib.sha256(data).hexdigest()
        self.local_storage[cid] = data
        return cid

    async def get_content(self, cid: str) -> Optional[bytes]:
        return self.local_storage.get(cid)
