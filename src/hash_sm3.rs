use libsm;

use crate::Hasher;

pub struct HasherSM3 {
    data: Vec<u8>,
}

impl HasherSM3 {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
}

impl Hasher for HasherSM3 {
    fn push(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    fn finish(&self) -> Vec<u8> {
        let mut r = Vec::new();
        let h = libsm::sm3::hash::Sm3Hash::new(&self.data).get_hash();
        r.extend_from_slice(&h[..]);
        r
    }
}
