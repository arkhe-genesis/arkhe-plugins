with open('cathedral-arkhe-33t/Cargo.toml', 'r') as f:
    content = f.read()

content = content.replace('candle-core = { version = "0.7", optional = true, features = ["cuda"] }\ncandle = { version = "0.7", optional = true }', 'candle-core = { version = "0.7", optional = true, features = ["cuda"] }')
content = content.replace('tensor-backend-candle = ["dep:candle"]', 'tensor-backend-candle = ["dep:candle-core"]')
content = content.replace('tensor-backend-metal = ["candle/metal"]', 'tensor-backend-metal = ["candle-core/metal"]')
content = content.replace('tensor-backend-directml = ["candle/directml"]', 'tensor-backend-directml = []')
content = content.replace('tensor-backend-nnapi = ["candle/nnapi"]', 'tensor-backend-nnapi = []')

with open('cathedral-arkhe-33t/Cargo.toml', 'w') as f:
    f.write(content)
