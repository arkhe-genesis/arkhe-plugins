with open('cathedral-arkhe-33t/src/lib.rs', 'r') as f:
    content = f.read()

content = content.replace('#![cfg_attr(not(feature = "std"), no_std)]\nextern crate std;', '#![cfg_attr(not(feature = "std"), no_std)]')

with open('cathedral-arkhe-33t/src/lib.rs', 'w') as f:
    f.write(content)

with open('cathedral-arkhe-33t/Cargo.toml', 'r') as f:
    content = f.read()

content = content.replace('default = ["tensor-backend-ndarray"]', 'default = ["tensor-backend-ndarray", "std"]\nstd = []')

with open('cathedral-arkhe-33t/Cargo.toml', 'w') as f:
    f.write(content)
