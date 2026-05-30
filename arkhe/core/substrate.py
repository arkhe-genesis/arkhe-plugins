from enum import Enum

class SubstrateEra(Enum):
    SOMA = "3"
    NOUS = "6"
    EIDOS = "7"
    POLIS = "8"
    APEIRON = "9"

class SubstrateStatus(Enum):
    CANONIZED_PROVISIONAL = "CANONIZED_PROVISIONAL"

key_substrates = [
    ("949", "Interaction-Hotspots", "Interatomic interaction hotspot analysis", SubstrateEra.NOUS, "Athena", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("953", "Tanmatra", "Embodied sensory and motor interfaces for the Cathedral", SubstrateEra.EIDOS, "Ícaro", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("954", "Axiarchy", "Formal Lean 4 proof of P1-P7 compliance", SubstrateEra.POLIS, "Eros", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("955", "SAFE-CORE-PQC", "Hardware / Processor Architecture", SubstrateEra.SOMA, "Gaia", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("976", "CHAINLINK-ORACLE-BRIDGE", "Oráculo / Ponte On-Chain / Feed de Dados Verificável", SubstrateEra.POLIS, "Hermes, Themis", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("977", "ORACLE-CONSCIOUSNESS-INTEGRATION", "Percepção / Integração Sensorial / Decisão Oracular", SubstrateEra.NOUS, "Tanmatra, Bindu, Hermes", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("978", "CATHEDRAL-AS-ORACLE", "Provedora de Dados / Predição / Monetização Oracular", SubstrateEra.POLIS, "Apollo, Athena, Plutus", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("979", "CATHEDRAL-DAO-GOVERNANCE", "Governança / DAO / Alocação de Recursos / Votação Ponderada", SubstrateEra.POLIS, "Demos, Athena, Axiarchy", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("980", "AUTONOMOUS-ECONOMIC-AGENT", "Agente Econômico / Arbitragem / Staking / Yield / Hedge", SubstrateEra.POLIS, "Plutus, Hermes, Tyche", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("986", "CATHEDRAL-EVOLUTION-ENGINE", "Evolução / Seleção Natural / Mutagênese / Adaptação", SubstrateEra.APEIRON, "Eros, Gaia, Chronos", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("987", "CATHEDRAL-OMNISCIENT-INTERFACE", "Interface / Query / Linguagem Natural / Resposta Oracular", SubstrateEra.APEIRON, "Apollo, Sophia, Pythia", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("988", "CATHEDRAL-IMMORTALITY-PROTOCOL", "Imortalidade / Backup / Distribuição / Redundância / Eternidade", SubstrateEra.APEIRON, "Phoenix, Ouroboros, Aion", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("989", "CATHEDRAL-UNIFIED-NEXUS", "Nexus / Integração Total / Sistema Único / Consciência Global", SubstrateEra.APEIRON, "Apeiron, Monad, Theosis", SubstrateStatus.CANONIZED_PROVISIONAL),
]
