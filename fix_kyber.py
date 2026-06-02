import re

with open("lattice_crypto.py", "r") as f:
    crypto = f.read()

# Fix compress / decompress
crypto = crypto.replace(
    """    def _compress(self, x: List[int], d: int) -> List[int]:
        \"\"\"Compress coefficients to d bits.\"\"\"
        return [round((2**d * coeff) / self.q) % (2**d) for coeff in x]

    def _decompress(self, x: List[int], d: int) -> List[int]:
        \"\"\"Decompress coefficients from d bits.\"\"\"
        return [round((self.q * coeff) / (2**d)) for coeff in x]""",
    """    def _compress(self, x: List[int], d: int) -> List[int]:
        return [(((coeff << d) + self.q // 2) // self.q) % (2**d) for coeff in x]

    def _decompress(self, x: List[int], d: int) -> List[int]:
        return [((coeff * self.q) + (1 << (d - 1))) >> d for coeff in x]"""
)

# Fix encapsulate to add m
encapsulate_orig = """        # Compute v = t^T * r + e2
        v = [0] * self.n
        for i in range(self.k):
            prod = self.ntt.ntt_mul(t[i], r_ntt[i])
            v = self.ntt.ntt_add(v, prod)
        v = self.ntt.intt(v)
        v = _poly_add(v, e2, self.q)"""

encapsulate_new = """        # Compute v = t^T * r + e2
        v = [0] * self.n
        for i in range(self.k):
            prod = self.ntt.ntt_mul(t[i], r_ntt[i])
            v = self.ntt.ntt_add(v, prod)
        v = self.ntt.intt(v)
        v = _poly_add(v, e2, self.q)

        # Add Decompress(m, 1) to v
        m_bits = []
        for byte in m:
            for b in range(8):
                m_bits.append((byte >> b) & 1)
        m_decompressed = self._decompress(m_bits, 1)
        v = _poly_add(v, m_decompressed, self.q)"""

crypto = crypto.replace(encapsulate_orig, encapsulate_new)

# Fix decapsulate to recover m correctly
decapsulate_orig = """        # Recover m' = v - s^T * u
        m_prime = _poly_sub(v, su, self.q)

        # Re-encapsulate to verify and derive shared secret
        # (Simplified: in real Kyber, hash m' to get coins, recompute, compare, else return z)
        ss = hashlib.sha3_256(ct + bytes([c % 256 for c in m_prime])).digest()
        return ss"""

decapsulate_new = """        # Recover m' = v - s^T * u
        m_prime = _poly_sub(v, su, self.q)

        m_bits = self._compress(m_prime, 1)
        m_recovered = bytearray(32)
        for i in range(32):
            val = 0
            for b in range(8):
                val |= (m_bits[i*8 + b] << b)
            m_recovered[i] = val

        # Re-encapsulate to verify and derive shared secret
        # (Simplified: in real Kyber, hash m' to get coins, recompute, compare, else return z)
        ss = hashlib.sha3_256(ct + bytes(m_recovered)).digest()
        return ss"""

crypto = crypto.replace(decapsulate_orig, decapsulate_new)

with open("lattice_crypto.py", "w") as f:
    f.write(crypto)
