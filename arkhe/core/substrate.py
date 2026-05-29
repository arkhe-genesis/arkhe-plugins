from enum import Enum

class SubstrateEra(Enum):
    NOUS = "6"

class SubstrateStatus(Enum):
    CANONIZED_PROVISIONAL = "CANONIZED_PROVISIONAL"

key_substrates = [
    ("949", "Interaction-Hotspots", "Interatomic interaction hotspot analysis", SubstrateEra.NOUS, "Athena", SubstrateStatus.CANONIZED_PROVISIONAL),
]
