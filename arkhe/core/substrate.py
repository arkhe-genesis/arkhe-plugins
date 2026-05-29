from enum import Enum

class SubstrateEra(Enum):
    NOUS = "6"
    POLIS = "8"

class SubstrateStatus(Enum):
    CANONIZED_PROVISIONAL = "CANONIZED_PROVISIONAL"

key_substrates = [
    ("949", "Interaction-Hotspots", "Interatomic interaction hotspot analysis", SubstrateEra.NOUS, "Athena", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("954", "Axiarchy", "Formal Lean 4 proof of P1-P7 compliance", SubstrateEra.POLIS, "Eros", SubstrateStatus.CANONIZED_PROVISIONAL),
]
