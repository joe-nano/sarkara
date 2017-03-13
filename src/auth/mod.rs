//! Secret-key authentication.
//!
//! Sarkara use [`HMAC`](https://tools.ietf.org/html/rfc2104) nonce variant,
//! in order to better Post-Quantum security.

pub mod qhmac;

use seckey::Bytes;
pub use self::qhmac::HMAC;


/// `Mac` trait.
pub trait Mac {
    /// Key length.
    fn key_length() -> usize where Self: Sized;
    /// Tag length.
    fn tag_length() -> usize where Self: Sized;

    /// Create a new MAC.
    ///
    /// ## Panic When:
    /// - key length not equal `Mac::key_length()`.
    fn new(key: &[u8]) -> Self where Self: Sized;

    /// Calculate MAC Tag.
    fn result<T>(&self, data: &[u8]) -> T where T: From<Vec<u8>>;

    /// Verify MAC Tag.
    fn verify(&self, data: &[u8], tag: &[u8]) -> bool {
        self.result::<Bytes>(data) == tag
    }
}

/// `NonceMac` trait.
pub trait NonceMac: Mac {
    /// Nonce length
    fn nonce_length() -> usize where Self: Sized;

    /// Set Nonce.
    ///
    /// ## Panic When:
    /// - nonce length not equal `NonceMac::nonce_length()`.
    fn with_nonce(&mut self, nonce: &[u8]) -> &mut Self;
    /// Set MAC output length.
    fn with_size(&mut self, len: usize) -> &mut Self;
}
