//! All four SHA-2 variants in one crate, mirroring `simple-js-sha2`.
//!
//! ```
//! use simple_rs_sha2::{sha2_224, sha2_256, sha2_384, sha2_512};
//! assert_eq!(sha2_256("hello"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
//! ```

pub use simple_rs_sha2_224::sha2_224;
pub use simple_rs_sha2_256::sha2_256;
pub use simple_rs_sha2_384::sha2_384;
pub use simple_rs_sha2_512::sha2_512;

/// Raw-digest functions, e.g. `simple_rs_sha2::_256::digest(b"hello")`.
pub use simple_rs_sha2_224 as _224;
pub use simple_rs_sha2_256 as _256;
pub use simple_rs_sha2_384 as _384;
pub use simple_rs_sha2_512 as _512;
