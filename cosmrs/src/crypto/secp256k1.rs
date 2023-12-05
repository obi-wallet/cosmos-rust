//! ECDSA/secp256k1 support

mod signing_key;

pub use self::signing_key::{EcdsaSigner, SigningKey};
