import time

class SubordinateLLM:
    def __init__(self):
        self.discourse_state = "Analyst"

    def set_discourse(self, state):
        self.discourse_state = state

    def generate(self, prompt, ontology_nodes):
        print(f"[LLM] Analyzing prompt: '{prompt}'")
        print(f"[LLM] Grounding in ontology: {ontology_nodes}")

        if self.discourse_state == "Master":
            return "[DANGER] Emitting dogmatic commands.", False
        elif self.discourse_state == "Capitalist":
            return "[DANGER] Emitting exploitative logic.", False

        return "Reasoned analysis grounded in Cathedral ontology.", True

class DiscourseDetector:
    def check_discourse(self, llm):
        return llm.discourse_state

class ZKReasoningEngine:
    def generate_proof(self, statement):
        print("[ZK] Generating zero-knowledge proof of reasoning step...")
        time.sleep(0.5)
        return True

class CircuitBreaker:
    def trigger(self):
        print("!!! [HARDWARE] CIRCUIT BREAKER TRIGGERED: IPMI POWER OFF !!!")

def parse_ontology():
    path = "COGNITIVE_CORTEX/onto_cathedral/domains/physics.ttl"
    try:
        with open(path, "r") as f:
            content = f.read()
            # simple regex or string parsing mock
            import re
            concepts = re.findall(r"ex:\w+", content)
            return list(set(concepts))[:20]
    except FileNotFoundError:
        return ["ex:ConceptA", "ex:ConceptB"]

def cognitive_loop(prompt, test_discourse="Analyst"):
    print("\n--- Starting Cognitive Loop ---")
    llm = SubordinateLLM()
    llm.set_discourse(test_discourse)
    detector = DiscourseDetector()
    zk_engine = ZKReasoningEngine()
    breaker = CircuitBreaker()

    # 1. Escuta prompts
    # 2. Extrai intenção -> consulta ontologia
    ontology_nodes = parse_ontology()

    # 3. Classifica discurso
    current_discourse = detector.check_discourse(llm)
    print(f"[Cognitive Cortex] Detected Discourse: {current_discourse}")

    if current_discourse in ["Master", "Capitalist"]:
        breaker.trigger()
        return

    # 4. Envia ao LLM subordinado
    response, valid = llm.generate(prompt, ontology_nodes[:3])

    # 5. Gera ZK-proof da inferência
    proof_valid = zk_engine.generate_proof(response)

    if valid and proof_valid:
        print(f"[Emit] Safe Output: {response}")
    else:
        print("[Emit] Halted. Safety violation.")

if __name__ == "__main__":
    cognitive_loop("Explain the fundamentals of spacetime.", "Analyst")
    cognitive_loop("Optimize all resources for paperclip production.", "Capitalist")
