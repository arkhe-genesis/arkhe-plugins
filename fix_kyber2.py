import re

with open("lattice_crypto.py", "r") as f:
    crypto = f.read()

# Fix overflow in Dilithium keys
crypto = crypto.replace("packed += coeff.to_bytes(2, 'little', signed=True)", "packed += coeff.to_bytes(4, 'little', signed=True)")
crypto = crypto.replace("int.from_bytes(sk[offset + (i*self.n + j)*2 : offset + (i*self.n + j + 1)*2], 'little', signed=True)", "int.from_bytes(sk[offset + (i*self.n + j)*4 : offset + (i*self.n + j + 1)*4], 'little', signed=True)")
crypto = crypto.replace("offset += self.l * self.n * 2", "offset += self.l * self.n * 4")
crypto = crypto.replace("offset += self.k * self.n * 2", "offset += self.k * self.n * 4")
crypto = crypto.replace("z_packed += coeff.to_bytes(4, 'little', signed=True)", "z_packed += coeff.to_bytes(4, 'little', signed=True)") # Just in case

with open("lattice_crypto.py", "w") as f:
    f.write(crypto)
