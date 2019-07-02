use blake2b_rs;

use crate::Hasher;

pub struct HasherBlake2B {
    k: Vec<u8>,
    data: Vec<u8>,
}

impl HasherBlake2B {
    pub fn new(k: Vec<u8>) -> Self {
        Self { k, data: vec![] }
    }
}

impl Hasher for HasherBlake2B {
    fn push(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    fn finish(&self) -> Vec<u8> {
        let mut dst = [0x00u8; 32];
        blake2b_rs::blake2b(&self.k, &self.data, &mut dst);
        dst.to_vec()
    }
}
