/// A Key Derivation Function that procudes a `Key` of KEY_SIZE bytes from the given key material, salt, and info.
pub trait Kdf {
    /// Derive a key from the given key material, salt, and info.
    fn kdf(ikm: &[u8], salt: &[u8], info: &[u8], okm: &mut [u8]);
}
