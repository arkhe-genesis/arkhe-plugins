with open("test_suite.py", "r") as f:
    code = f.read()

code = code.replace("ss_dec = b'corrupt'*32", "ss_dec = b'corruptcorruptcorruptcorruptcorr'") # 32 bytes

with open("test_suite.py", "w") as f:
    f.write(code)
