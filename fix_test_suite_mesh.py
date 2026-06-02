with open("test_suite.py", "r") as f:
    code = f.read()

code = code.replace("assert decrypted == msg", "assert decrypted == msg # Check decrypted")

# The issue in test_encrypted_communication might be that session_key is different for alice and bob?
# Let's check session keys.
