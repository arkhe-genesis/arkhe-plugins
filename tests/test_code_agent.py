from catedral_code_agent import CatedralCodeAgent

def test_init_session():
    agent = CatedralCodeAgent()
    session = agent.init_session(".", language="python", depth="surface")
    assert session.language == "python"
