# At the top of arkhe_os_v6.py
from arkhe.substrates.onchain_canonizer import (
    OnChainCanonizer, MetaOrchestratorBootVerifier,
    CanonizationType, EIP712Signer, ChainId
)
import os
import asyncio

# In the MetaOrchestrator class:
class MetaOrchestrator:
    def __init__(self):
        self.canonizer = OnChainCanonizer(
            api_key=os.getenv("ETHERSCAN_API_KEY"),
            private_key=os.getenv("CANONIZER_PRIVATE_KEY"),
            chain_id=ChainId.MAINNET
        )
        self.boot_verifier = MetaOrchestratorBootVerifier(self.canonizer)

    async def boot(self):
        # Initialize canonizer (signs kernel, syncs signatures)
        await self.canonizer.initialize()

        # Verify boot integrity
        boot_ok, errors = await self.boot_verifier.verify_boot()
        if not boot_ok:
            raise Exception(f"Canonical boot failed: {errors}")

        # Start continuous sync
        asyncio.create_task(self.canonizer.continuous_sync(interval_seconds=300))

        # Continue normal boot...

    def propose_policy_change(self, policy_type, params):
        """Kernel proposes a policy change for human approval"""
        return self.canonizer.propose_meta_orchestrator_policy(
            policy_type=policy_type,
            parameters=params
        )

    def get_canonical_proof(self, artifact_hash):
        """Get ZK-integrated proof for any canonized artifact"""
        return self.canonizer.get_canonization_proof(artifact_hash)
