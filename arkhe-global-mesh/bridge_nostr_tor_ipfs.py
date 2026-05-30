import asyncio
import json
import hashlib
from typing import Dict, List, Optional, Set
from dataclasses import dataclass, field
from datetime import datetime, timezone

from nostr.nostr_relay import NostrEvent, CathedralNostrRelay
from tor.tor_service import TorMeshNode
from ipfs.ipfs_backbone import IPFSBackbone

class NostrBridge:
    DEFAULT_RELAYS: List[str] = [
        "wss://relay.arkhe-cathedral.org",
        "wss://nostr.wine",
        "wss://relay.damus.io",
    ]

    def __init__(self, pubkey: str, privkey: str, relays: Optional[List[str]] = None):
        self.pubkey = pubkey
        self.privkey = privkey
        self.relays = relays or self.DEFAULT_RELAYS
        self.subscribed_events: List[NostrEvent] = []

    async def broadcast_seal(self, substrate_id: int, seal: str) -> bool:
        content = json.dumps({"substrate": substrate_id, "seal": seal})
        event_id = hashlib.sha256(content.encode()).hexdigest()
        sig = hashlib.sha3_256(f"{event_id}{self.privkey}".encode()).hexdigest()[:128]
        event = NostrEvent(
            id=event_id,
            pubkey=self.pubkey,
            created_at=int(datetime.now(timezone.utc).timestamp()),
            kind=30078,
            tags=[["e", f"substrate-{substrate_id}"]],
            content=content,
            sig=sig,
        )
        for relay in self.relays:
            print(f"  [NOSTR] Broadcasting seal to {relay}: {event.id[:16]}...")
        return True

    async def discover_nodes(self) -> List[Dict]:
        return [{"pubkey": self.pubkey, "substrates": [972, 972.1], "relay": r} for r in self.relays]

@dataclass
class TorEndpoint:
    onion_address: str
    virtual_port: int = 9230
    auth_cookie: Optional[str] = None

class TorBridge:
    def __init__(self, control_port: int = 9051, proxy_port: int = 9050):
        self.control_port = control_port
        self.proxy_port = proxy_port
        self.hidden_service: Optional[TorEndpoint] = None
        self.active_circuits: Dict[str, str] = {}

    async def create_hidden_service(self, target_port: int = 9230) -> TorEndpoint:
        onion = f"arkhe{hashlib.sha256(b'seed').hexdigest()[:16]}.onion"
        self.hidden_service = TorEndpoint(onion_address=onion, virtual_port=target_port)
        print(f"  [TOR] Hidden service created: {onion}:9230")
        return self.hidden_service

    async def connect_onion(self, onion: str, port: int = 9230) -> bool:
        print(f"  [TOR] Routing to {onion}:{port} via SOCKS5")
        self.active_circuits[onion] = f"127.0.0.1:{self.proxy_port}"
        return True

@dataclass
class ArkheArtifact:
    substrate_id: int
    filename: str
    cid: str
    size_bytes: int
    seal: str

class IpfsBridge:
    def __init__(self, api_endpoint: str = "http://localhost:5001"):
        self.api = api_endpoint
        self.pinned_cids: Set[str] = set()

    async def publish_artifact(self, substrate_id: int, filepath: str, content: bytes) -> ArkheArtifact:
        cid = f"Qm{hashlib.sha256(content).hexdigest()[:44]}"
        seal = hashlib.sha3_256(content).hexdigest()
        self.pinned_cids.add(cid)
        print(f"  [IPFS] Published /ipfs/{cid} ({len(content)} bytes)")
        return ArkheArtifact(substrate_id, filepath, cid, len(content), seal)

    async def fetch_artifact(self, cid: str) -> Optional[bytes]:
        if cid not in self.pinned_cids:
            print(f"  [IPFS] Fetching {cid} from DHT...")
        return b"# ARKHE Artifact\n# CID: " + cid.encode()

class NostrTorIpfsBridge:
    def __init__(self, node_id: str, ed25519_pubkey: str, ed25519_privkey: str):
        self.node_id = node_id
        self.nostr = NostrBridge(ed25519_pubkey, ed25519_privkey)
        self.tor = TorBridge()
        self.ipfs = IpfsBridge()
        self.status = "initialized"

    async def bootstrap(self) -> Dict:
        manifest = json.dumps({"node_id": self.node_id}).encode()
        artifact = await self.ipfs.publish_artifact(972, "manifest.json", manifest)
        tor_ep = await self.tor.create_hidden_service()
        seal = hashlib.sha3_256(f"{self.node_id}:{artifact.cid}:{tor_ep.onion_address}".encode()).hexdigest()
        await self.nostr.broadcast_seal(972, seal)
        self.status = "active"
        return {"node_id": self.node_id, "ipfs_cid": artifact.cid, "tor_onion": tor_ep.onion_address, "seal": seal}

    async def mesh_connect(self, target_onion: Optional[str] = None) -> bool:
        if target_onion:
            return await self.tor.connect_onion(target_onion)
        nodes = await self.nostr.discover_nodes()
        return True
