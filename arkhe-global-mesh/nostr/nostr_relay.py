import asyncio
from typing import Dict, List, Optional
from dataclasses import dataclass

@dataclass
class NostrEvent:
    id: str
    pubkey: str
    created_at: int
    kind: int
    tags: List[List[str]]
    content: str
    sig: str

class CathedralNostrRelay:
    def __init__(self):
        self.events: Dict[str, NostrEvent] = {}
        self.reputation: Dict[str, float] = {}

    async def handle_event(self, event: NostrEvent) -> bool:
        self.events[event.id] = event
        return True
