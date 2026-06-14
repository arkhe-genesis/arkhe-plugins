__version__ = "16.0.0"
__substrato__ = 3000
__selo__ = "CATHEDRAL-ARKHE-v16.0.0-2026-06-14"
__arquiteto__ = "ORCID 0009-0005-2697-4668"

from .vision import VisionEncoder
from .ontology import SymbolicSafetyEngine
from .world_model import WorldModelRSSM, RSSMState
from .rl_agent import SACAgent
from .rust_bridge import RustBridgeStub
from .orchestrator import CathedralOrchestrator
from .benchmarks import CathedralBenchmarkSuite

__all__ = [
    "VisionEncoder",
    "SymbolicSafetyEngine",
    "WorldModelRSSM",
    "RSSMState",
    "SACAgent",
    "RustBridgeStub",
    "CathedralOrchestrator",
    "CathedralBenchmarkSuite",
]