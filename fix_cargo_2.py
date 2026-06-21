with open('cathedral-arkhe-33t/Cargo.toml', 'r') as f:
    content = f.read()

content = content.replace('candle-core = { version = "0.7", optional = true, features = ["cuda"] }', 'candle = { version = "0.7", optional = true, features = ["cuda"] }')
content = content.replace('tensor-backend-candle = ["dep:candle-core"]', 'tensor-backend-candle = ["dep:candle"]')
content = content.replace('tensor-backend-metal = ["candle-core/metal"]', 'tensor-backend-metal = ["candle/metal"]')
content = content.replace('tensor-backend-directml = []', 'tensor-backend-directml = ["candle/directml"]')
content = content.replace('tensor-backend-nnapi = []', 'tensor-backend-nnapi = ["candle/nnapi"]')

with open('cathedral-arkhe-33t/Cargo.toml', 'w') as f:
    f.write(content)
