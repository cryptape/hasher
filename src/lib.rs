pub trait Hasher {
    fn push(&mut self, bytes: &[u8]);
    fn finish(&self) -> Vec<u8>;

    fn hash(&mut self, data: &[u8]) -> Vec<u8> {
        self.push(data);
        self.finish()
    }
}

#[cfg(feature = "hash-keccak")]
mod hash_keccak;
#[cfg(feature = "hash-keccak")]
pub use hash_keccak::HasherKeccak;

#[cfg(feature = "hash-blake2b")]
mod hash_blake2b;
#[cfg(feature = "hash-blake2b")]
pub use hash_blake2b::HasherBlake2B;

#[cfg(feature = "hash-sm3")]
mod hash_sm3;
#[cfg(feature = "hash-sm3")]
pub use hash_sm3::HasherSM3;
