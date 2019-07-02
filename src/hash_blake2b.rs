use blake2b_rs;

use crate::Hasher;

pub struct HasherBlake2B {
    k: Vec<u8>,
}

impl HasherBlake2B {
    pub fn new(k: Vec<u8>) -> Self {
        Self { k }
    }
}

impl Hasher for HasherBlake2B {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut dst = [0x00u8; 32];
        blake2b_rs::blake2b(&self.k, data, &mut dst);
        dst.to_vec()
    }
}
