class EpistemicCommitClient:
    def __init__(self): pass
    async def commit_state(self, req): return {"state_hash": b"hash", "temporal_event_id": "event_123"}
