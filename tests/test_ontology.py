from visual_ontology_engine import VisualOntologyEngine

def test_ingest():
    engine = VisualOntologyEngine("schema_943.jsonld")
    data = {
        "colors": {"sacred_gold": "#D4AF37"},
        "typography": {"serif_sacred": "Cinzel"}
    }
    res = engine.ingest_design_system(data)
    assert res["tokens_ingested"] == 2
