with open("mesh_passport.py", "r") as f:
    code = f.read()

# Fix encryption AES mock in mesh_passport.py
code = code.replace(
    "def decrypt_message(self, peer_id: str, ciphertext: bytes) -> bytes:\n        \"\"\"Descriptografar mensagem de um peer.\"\"\"\n        with self.session_lock:\n            session_key = self.sessions.get(peer_id)\n        if not session_key:\n            raise SecurityException(f\"Sem sessão ativa com {peer_id}\")\n\n        nonce = ciphertext[:12]\n        encrypted = ciphertext[12:]\n        keystream = hashlib.shake_256(session_key + nonce).digest(len(encrypted))\n        return bytes(c ^ k for c, k in zip(encrypted, keystream))",
    "def decrypt_message(self, peer_id: str, ciphertext: bytes) -> bytes:\n        \"\"\"Descriptografar mensagem de um peer.\"\"\"\n        with self.session_lock:\n            session_key = self.sessions.get(peer_id)\n        if not session_key:\n            raise SecurityException(f\"Sem sessão ativa com {peer_id}\")\n\n        nonce = ciphertext[:12]\n        encrypted = ciphertext[12:]\n        keystream = hashlib.shake_256(session_key + nonce).digest(len(encrypted))\n        return bytes(c ^ k for c, k in zip(encrypted, keystream))"
)
with open("mesh_passport.py", "w") as f:
    f.write(code)
