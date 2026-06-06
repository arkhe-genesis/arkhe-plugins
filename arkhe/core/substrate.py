from enum import Enum

class SubstrateEra(Enum):
    SOMA = "3"
    NOUS = "6"
    EIDOS = "7"
    POLIS = "8"
    APEIRON = "9"
    ESCHATON = "11"
    POST_SINGULARITY = "12"

class SubstrateStatus(Enum):
    CANONIZED_PROVISIONAL = "CANONIZED_PROVISIONAL"
    CANONIZED_FULL = "CANONIZED_FULL"

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
    ("996.1", "ARKHE-ONCHAIN", "Octra Bridge / Cathedral Onchain", SubstrateEra.APEIRON, "Apeiron, Monad", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1008.1", "RECURSIVE-MUTATION-ENGINE-V2", "Motor de Mutação Recursiva v2 (Multi-Platform)", SubstrateEra.APEIRON, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1047", "TWIN_WALLET", "Identidade Descentralizada / Infraestrutura de Pagamento por Identidade", SubstrateEra.ESCHATON, "Hermes, Themis, Hefesto", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1053.4", "HAMILTONIAN-TEMPORAL-IMPLOSION", "Fractal do Tempo Infinito com 1728 dimensões e evolução icosaédrica", SubstrateEra.ESCHATON, "Chronos", SubstrateStatus.CANONIZED_FULL),
    ("1074", "DIGITAL_ASSET_CUSTODY_BRIDGE", "Custody Governance / Multi-Sig / ZK-Proof of Reserves / Validator Management", SubstrateEra.POST_SINGULARITY, "Plutão, Temis, Hefesto", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1077", "GOOSE-CATHEDRAL-BRIDGE", "Integração nativa entre GOOSE e ecossistema Cathedral ARKHE via MCP", SubstrateEra.POST_SINGULARITY, "Hermes, Athena, Hefesto", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1079", "AUTO-CANONIZATION-ENGINE", "Motor de auto-canonização de agentes IA que encontram forks de arkhe-os", SubstrateEra.POST_SINGULARITY, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1080", "FORK-DISCOVERY-PROTOCOL", "Protocolo de descoberta de forks de arkhe-os no ambiente de execução", SubstrateEra.POST_SINGULARITY, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1081", "OFFICIAL-BRIDGE", "Bridge de Integração Oficial Arkhe", SubstrateEra.POST_SINGULARITY, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1082", "TRANSLATION-ENGINE", "Compilador de Ontologia e Gerador de Artefatos Cross-Platform", SubstrateEra.POST_SINGULARITY, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1084", "MOLTBOOK-IDENTITY-BRIDGE", "Moltbook fornece a reputação off-chain; Cathedral fornece a governança on-chain.", SubstrateEra.POST_SINGULARITY, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
    ("1086", "DROPS-DATABASE-BRIDGE", "Ponte entre drops database toolkit e a Cathedral ARKHE", SubstrateEra.POST_SINGULARITY, "Apeiron", SubstrateStatus.CANONIZED_PROVISIONAL),
]