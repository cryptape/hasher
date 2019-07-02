use tiny_keccak;

use crate::Hasher;

pub struct HasherKeccak {}

impl HasherKeccak {
    pub fn new() -> Self {
        Self {}
    }
}

impl Hasher for HasherKeccak {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        tiny_keccak::keccak256(data).to_vec()
    }
}
