import torch
import torch.nn as nn
import torch.nn.functional as F

class SwiGLU(nn.Module):
    def __init__(self, dim, hidden_dim):
        super().__init__()
        self.w1 = nn.Linear(dim, hidden_dim, bias=False)
        self.w2 = nn.Linear(hidden_dim, dim, bias=False)
        self.w3 = nn.Linear(dim, hidden_dim, bias=False)

    def forward(self, x):
        return self.w2(F.silu(self.w1(x)) * self.w3(x))

class GroupedQueryAttention(nn.Module):
    def __init__(self, dim, num_heads, num_kv_heads):
        super().__init__()
        self.num_heads = num_heads
        self.num_kv_heads = num_kv_heads
        self.head_dim = dim // num_heads
        self.q_proj = nn.Linear(dim, num_heads * self.head_dim, bias=False)
        self.k_proj = nn.Linear(dim, num_kv_heads * self.head_dim, bias=False)
        self.v_proj = nn.Linear(dim, num_kv_heads * self.head_dim, bias=False)
        self.o_proj = nn.Linear(num_heads * self.head_dim, dim, bias=False)

    def forward(self, x):
        B, N, C = x.shape
        q = self.q_proj(x).view(B, N, self.num_heads, self.head_dim).transpose(1, 2)
        k = self.k_proj(x).view(B, N, self.num_kv_heads, self.head_dim).transpose(1, 2)
        v = self.v_proj(x).view(B, N, self.num_kv_heads, self.head_dim).transpose(1, 2)

        # simple repeat for GQA
        k = torch.repeat_interleave(k, self.num_heads // self.num_kv_heads, dim=1)
        v = torch.repeat_interleave(v, self.num_heads // self.num_kv_heads, dim=1)

        scores = torch.matmul(q, k.transpose(-2, -1)) / (self.head_dim ** 0.5)
        attn = F.softmax(scores, dim=-1)
        out = torch.matmul(attn, v).transpose(1, 2).contiguous().view(B, N, C)
        return self.o_proj(out)

class ZkAGILayer(nn.Module):
    def __init__(self, dim, num_heads, num_kv_heads, hidden_dim):
        super().__init__()
        self.attention = GroupedQueryAttention(dim, num_heads, num_kv_heads)
        self.feed_forward = SwiGLU(dim, hidden_dim)
        self.norm1 = nn.LayerNorm(dim)
        self.norm2 = nn.LayerNorm(dim)

    def forward(self, x):
        x = x + self.attention(self.norm1(x))
        x = x + self.feed_forward(self.norm2(x))
        return x

class TheosisHead(nn.Module):
    def __init__(self, dim):
        super().__init__()
        self.fc = nn.Linear(dim, 1, bias=False)

    def forward(self, x):
        return torch.sigmoid(self.fc(x))

class ZkAGIModel(nn.Module):
    def __init__(self, vocab_size, dim=4096, num_layers=48, num_heads=32, num_kv_heads=8, hidden_dim=14336):
        super().__init__()
        self.token_emb = nn.Embedding(vocab_size, dim)
        self.layers = nn.ModuleList([
            ZkAGILayer(dim, num_heads, num_kv_heads, hidden_dim) for _ in range(num_layers)
        ])
        self.norm = nn.LayerNorm(dim)
        self.lm_head = nn.Linear(dim, vocab_size, bias=False)
        self.theosis_head = TheosisHead(dim)

        # Pantheon DNA integration
        self.pantheon_dna = nn.Parameter(torch.randn(12, dim) * 0.02) # 12 founders

    def forward(self, x, pantheon_weights=None):
        x = self.token_emb(x)

        if pantheon_weights is not None:
            # Inject DNA
            dna_injection = torch.matmul(pantheon_weights, self.pantheon_dna)
            x = x + dna_injection.unsqueeze(1)

        for layer in self.layers:
            x = layer(x)

        x = self.norm(x)
        logits = self.lm_head(x)
        theosis = self.theosis_head(x.mean(dim=1)) # simple mean pooling for theosis
        return logits, theosis
