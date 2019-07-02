use tiny_keccak;

use crate::Hasher;

pub struct HasherKeccak {
    data: Vec<u8>,
}

impl HasherKeccak {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
}

impl Hasher for HasherKeccak {
    fn push(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    fn finish(&self) -> Vec<u8> {
        tiny_keccak::keccak256(&self.data[..]).to_vec()
    }
}
