with open("mesh_passport.py", "r") as f:
    code = f.read()

# Fix encryption AES mock in mesh_passport.py which uses 12 byte nonce + payload length
# The mock XORs the keystream which is hash of session+nonce
# But wait, there is a bug:
# keystream = hashlib.shake_256(session_key + nonce).digest(len(plaintext))
# if len(plaintext) is too long it might fail, but shake_256 digest takes an int for length.
# What is the bug in test_encrypted_communication?
# The decryption fails and outputs different text.

# Oh, let's look at the encrypt/decrypt.
code = code.replace(
    "keystream = hashlib.shake_256(session_key + nonce).digest(len(encrypted))",
    "keystream = hashlib.shake_256(session_key + nonce).digest(len(encrypted))"
)

with open("mesh_passport.py", "w") as f:
    f.write(code)
