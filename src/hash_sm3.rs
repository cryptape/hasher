use libsm;

use crate::Hasher;

pub struct HasherSM3 {}

impl HasherSM3 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Hasher for HasherSM3 {
    const LENGTH: usize = 32;

    fn digest(&self, data: &[u8]) -> Vec<u8> {
        let mut r = Vec::new();
        let h = libsm::sm3::hash::Sm3Hash::new(data).get_hash();
        r.extend_from_slice(&h[..]);
        r
    }
}
