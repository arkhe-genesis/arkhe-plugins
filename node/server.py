import asyncio
import os
from passport_gateway import PassportGateway
from api_gateway import APIGateway

class ArkheNode:
    def __init__(self, config_path: str = "config.yaml"):
        self.node_id = "node-1"
        self.config = {"passport_enabled": True}
        self.passport = PassportGateway()
        self.api = APIGateway(node_id=self.node_id, passport=self.passport)

    async def start(self):
        print(f"Starting ArkheNode {self.node_id}...")
        if self.config.get("passport_enabled", True):
            await self.passport.start()
        # await self.api.start_http_server()
        print("ArkheNode started.")

    async def stop(self):
        print(f"Stopping ArkheNode {self.node_id}...")
        await self.passport.stop()
        self.api.stop()
        print("ArkheNode stopped.")

if __name__ == "__main__":
    node = ArkheNode()
    asyncio.run(node.start())
    asyncio.run(node.stop())
