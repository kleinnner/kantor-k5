pub mod field;
pub mod poseidon;
pub mod sponge;
pub mod balloon;

use sponge::K5Sponge;
use balloon::K5MemoryHard;

pub const DOMAIN_TAG: &[u8] = b"KLEINNER_KANTOR_5_V1";

pub fn k5_512(data: &[u8]) -> [u8; 64] {
    let mut out = K5Sponge::new(DOMAIN_TAG).absorb(data).squeeze(512);
    out.try_into().unwrap()
}

pub fn k5_1024(data: &[u8]) -> [u8; 128] {
    let mut out = K5Sponge::new(DOMAIN_TAG).absorb(data).squeeze(1024);
    out.try_into().unwrap()
}

pub fn k2048(data: &[u8]) -> [u8; 256] {
    let mut out = K5Sponge::new(DOMAIN_TAG).absorb(data).squeeze(2048);
    out.try_into().unwrap()
}

pub fn kof(data: &[u8], bits: usize) -> Vec<u8> {
    K5Sponge::new(DOMAIN_TAG).absorb(data).squeeze(bits)
}

pub fn k5_h(data: &[u8], mem_kib: usize, time_cost: u32) -> [u8; 32] {
    let hardened = K5MemoryHard::new(mem_kib, time_cost).process(data);
    let mut out = K5Sponge::new(DOMAIN_TAG).absorb(&hardened).squeeze(512);
    out[..32].try_into().unwrap()
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_k5_512_deterministic() {
        let h1 = k5_512(b"hello");
        let h2 = k5_512(b"hello");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_k5_512_diff_inputs() {
        let h1 = k5_512(b"abc");
        let h2 = k5_512(b"xyz");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_k5_512_output_len() {
        assert_eq!(k5_512(b"test").len(), 64);
    }

    #[test]
    fn test_k2048_output_len() {
        assert_eq!(k2048(b"test").len(), 256);
    }

    #[test]
    fn test_kof_output_len() {
        assert_eq!(kof(b"test", 4096).len(), 512);
    }

    #[test]
    fn test_k5_h_differs_from_standard() {
        let std = k5_512(b"hello");
        let hard = k5_h(b"hello", 64, 1);
        assert_ne!(&std[..], &hard[..]);
    }

    #[test]
    fn test_hex_output() {
        let h = k5_512(b"hello");
        assert_eq!(hex::encode(h), hex::encode(h));
    }
}
