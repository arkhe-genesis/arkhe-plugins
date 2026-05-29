class TemporalChainClient:
    def __init__(self): pass
    async def commit_event(self, req): return {"status": "ANCHORED_L2", "l2_tx_hash": "tx_123"}
