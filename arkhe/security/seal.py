import hashlib
import json
from typing import Any

class Seal:
    def compute(self, payload: Any) -> str:
        s = json.dumps(payload, sort_keys=True)
        return hashlib.sha3_256(s.encode()).hexdigest()
