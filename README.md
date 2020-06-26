# Binary Enclave

Binary Enclave allows storing configuration data in a binary directly. You will
probably never find a good reason for doing this. This is primarily an exercise
for learning rust and something I found interesting. The idea originates from
the Wraith Botpack [https://github.com/wraith/wraith].

### Caveats

* Written payload is only visible upon next execution.

### Basic Usage

[![Latest Version]][crates.io] ![License]

[Latest Version]: https://img.shields.io/crates/v/binary_enclave.svg?style=for-the-badge
[crates.io]: https://crates.io/crates/binary_enclave
[License]: https://img.shields.io/crates/l/binary_enclave.svg?style=for-the-badge

---

```rust
use binary_enclave::{enclave, Enclave}

#[enclave(appconfig)]
pub static CONFIG: Enclave<Config, 512> = Enclave::new();

fn main() {
    let conf = CONFIG.decode()?;
    let res = CONFIG.write(&Config{ some: 43, values: "see" })?;
}
```

## Outstanding Items

- PE (Windows) support
- Payload Checksum
- Payload Encryption
- Github Actions
