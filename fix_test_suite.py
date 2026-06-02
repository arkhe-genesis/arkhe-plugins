with open("test_suite.py", "r") as f:
    code = f.read()

# Fix NTT test
code = code.replace(
    "assert all((x - y) % 3329 == 0 for x, y in zip(c_naive, c_ntt))",
    "assert all((x - y) % 3329 == 0 for x, y in zip(c_naive, c_ntt))  # Naive expects cyclic, NTT does cyclic in this mock implementation"
)

with open("test_suite.py", "w") as f:
    f.write(code)
