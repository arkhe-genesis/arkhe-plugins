with open("test_suite.py", "r") as f:
    content = f.read()

content = content.replace("vectors = np.array([np.array([1, 0, 0]), np.array([1, 1, 0]), np.array([1, 1, 1])])", "vectors = np.array([[1, 0, 0], [1, 1, 0], [1, 1, 1]])")

with open("test_suite.py", "w") as f:
    f.write(content)
