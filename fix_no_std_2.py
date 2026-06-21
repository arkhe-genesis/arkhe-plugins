import re
with open('cathedral-arkhe-33t/src/lib.rs', 'r') as f:
    content = f.read()

content = re.sub(r'extern crate std;\s*extern crate std;', 'extern crate std;', content)

with open('cathedral-arkhe-33t/src/lib.rs', 'w') as f:
    f.write(content)
