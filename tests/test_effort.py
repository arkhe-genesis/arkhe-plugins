from cognitive_effort_controller import CognitiveEffortController, TaskProfile

def test_effort_levels():
    ctrl = CognitiveEffortController("", "")
    task = TaskProfile("1", "test", "text", 100, 1, "general")
    effort = ctrl.compute_effort(task)
    assert effort.level.value in ["low", "medium", "high", "extra_high", "max"]
