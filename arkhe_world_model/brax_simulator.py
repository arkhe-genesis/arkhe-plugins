from dataclasses import dataclass
import numpy as np

@dataclass
class SimulationConfig:
    pass

class ArkheBraxSimulator:
    def __init__(self, scene="pendulum"):
        self.scene = scene

    def reset(self, seed=None):
        return {"x": np.zeros(3), "qd": np.zeros(6)}

    def get_trajectory_embedding(self, window=5):
        return np.zeros((256,), dtype=np.float32)

    def step(self, state, action):
        return state

    def get_world_embedding(self, state):
        return np.zeros((256,), dtype=np.float32)
