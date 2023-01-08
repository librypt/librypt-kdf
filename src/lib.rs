/// A derived key represented by a fixed array of bytes.
pub type Key<const SIZE: usize> = [u8; SIZE];

/// A Key Derivation Function that procudes a `Key` of KEY_SIZE bytes from the given key material, salt, and info.
pub trait Kdf {
    /// Derive a key of KEY_SIZE bytes from the given key material, salt, and info.
    fn kdf<const KEY_SIZE: usize>(km: &[u8], salt: &[u8], info: &[u8]) -> Key<KEY_SIZE>;
}
