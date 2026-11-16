Rust port of [simple-js-sha2](https://github.com/Azero123/simple-js-sha2): SHA-2 224, 256, 384 and 512 in one crate.

```toml
[dependencies]
simple-rs-sha2 = "1"
```
```rust
use simple_rs_sha2::{sha2_224, sha2_256, sha2_384, sha2_512};

sha2_256("hello"); // 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
simple_rs_sha2::_512::digest(b"hello"); // raw [u8; 64]
```
