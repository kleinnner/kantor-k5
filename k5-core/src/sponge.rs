use crate::field::Goldilocks;
use crate::poseidon::{Poseidon, T};

pub const RATE_BYTES: usize = 16;

pub fn pad_block(data: &[u8]) -> Vec<u8> {
    let data_len_bits = (data.len() as u64) * 8;
    let mut padded = data.to_vec();
    padded.push(0x80);
    while (padded.len() + 8) % RATE_BYTES != 0 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&data_len_bits.to_be_bytes());
    padded
}

pub struct K5Sponge {
    poseidon: Poseidon,
    pub state: [Goldilocks; T],
    buffer: Vec<u8>,
    finalized: bool,
    domain_tag: Vec<u8>,
}

impl K5Sponge {
    pub fn new(domain_tag: &[u8]) -> Self {
        Self {
            poseidon: Poseidon::new(domain_tag),
            state: [Goldilocks::zero(); T],
            buffer: Vec::new(),
            finalized: false,
            domain_tag: domain_tag.to_vec(),
        }
    }

    pub fn absorb(&mut self, data: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(data);
        while self.buffer.len() >= RATE_BYTES {
            let block: [u8; 16] = self.buffer[..16].try_into().unwrap();
            self.buffer.drain(..16);
            self._absorb_block(&block);
        }
        self
    }

    fn _absorb_block(&mut self, block: &[u8; 16]) {
        let e0 = Goldilocks::from_bytes(&block[..8]);
        let e1 = Goldilocks::from_bytes(&block[8..]);
        self.state[0] = self.state[0].add(e0);
        self.state[1] = self.state[1].add(e1);
        self.state = self.poseidon.permute(&self.state);
    }

    fn finalize(&mut self) {
        if self.finalized {
            return;
        }
        self.finalized = true;
        let padded = pad_block(&self.buffer);
        self.buffer.clear();
        for chunk in padded.chunks(RATE_BYTES) {
            let block: [u8; 16] = chunk.try_into().unwrap();
            self._absorb_block(&block);
        }
    }

    pub fn squeeze(&mut self, bits: usize) -> Vec<u8> {
        if bits == 0 {
            return Vec::new();
        }
        if bits % 8 != 0 {
            panic!("bits must be multiple of 8");
        }
        self.finalize();
        if bits > 4096 {
            return self._squeeze_xof(bits);
        }
        self._squeeze_direct(bits)
    }

    fn _squeeze_direct(&mut self, bits: usize) -> Vec<u8> {
        let needed = bits / 8;
        let mut out = Vec::with_capacity(needed);
        while out.len() < needed {
            out.extend_from_slice(&self.state[0].to_bytes());
            out.extend_from_slice(&self.state[1].to_bytes());
            if out.len() < needed {
                self.state = self.poseidon.permute(&self.state);
            }
        }
        out.truncate(needed);
        out
    }

    fn _squeeze_xof(&mut self, bits: usize) -> Vec<u8> {
        use sha3::{Shake256, digest::{ExtendableOutput, Update, XofReader}};
        let seed = [self.state[0].to_bytes(), self.state[1].to_bytes()].concat();
        let mut hasher = Shake256::default();
        hasher.update(&seed);
        hasher.update(&self.domain_tag);
        let mut reader = hasher.finalize_xof();
        let mut out = vec![0u8; bits / 8];
        reader.read(&mut out);
        out
    }

    pub fn reset(&mut self) -> &mut Self {
        self.state = [Goldilocks::zero(); T];
        self.buffer.clear();
        self.finalized = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sponge_deterministic() {
        let mut s = K5Sponge::new(b"KLEINNER_KANTOR_5_V1");
        let r1 = s.absorb(b"hello").squeeze(512);
        s.reset();
        let r2 = s.absorb(b"hello").squeeze(512);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_sponge_streaming() {
        let mut s = K5Sponge::new(b"KLEINNER_KANTOR_5_V1");
        s.absorb(b"hello ").absorb(b"world");
        let r1 = s.squeeze(512);
        s.reset();
        let r2 = s.absorb(b"hello world").squeeze(512);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_sponge_output_len() {
        let mut s = K5Sponge::new(b"KLEINNER_KANTOR_5_V1");
        for bits in [256, 512, 1024, 2048] {
            s.reset();
            let out = s.absorb(b"test").squeeze(bits);
            assert_eq!(out.len(), bits / 8);
        }
    }

    #[test]
    fn test_sponge_domain_separation() {
        let mut s1 = K5Sponge::new(b"DOMAIN_A");
        let mut s2 = K5Sponge::new(b"DOMAIN_B");
        let r1 = s1.absorb(b"data").squeeze(512);
        let r2 = s2.absorb(b"data").squeeze(512);
        assert_ne!(r1, r2);
    }
}
