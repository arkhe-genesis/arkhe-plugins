class QUICMeshClient:
    def __init__(self): pass
    async def open_channel(self, channel, channel_type): pass
    async def publish(self, packet, quorum_ack, quorum_size): return {"quorum_reached": True, "replicas_ack": quorum_size}
