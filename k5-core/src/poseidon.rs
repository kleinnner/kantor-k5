use sha3::{Shake256, digest::{ExtendableOutput, Update, XofReader}};

use crate::field::{Goldilocks, GOLDILOCKS_PRIME};

pub const T: usize = 4;
pub const FULL_ROUNDS: usize = 8;
pub const PARTIAL_ROUNDS: usize = 28;
pub const TOTAL_ROUNDS: usize = FULL_ROUNDS + PARTIAL_ROUNDS;
pub const HALF_FULL: usize = FULL_ROUNDS / 2;
pub const SBOX_ALPHA: u64 = 7;

#[derive(Clone)]
pub struct Poseidon {
    pub round_constants: Vec<[Goldilocks; T]>,
    pub mds: [[Goldilocks; T]; T],
}

fn cauchy_mds() -> [[Goldilocks; T]; T] {
    let mut mds = [[Goldilocks::zero(); T]; T];
    for i in 0..T {
        for j in 0..T {
            let x = (i + 1) as u64;
            let y = (j + T + 1) as u64;
            let denom = (x + y) % GOLDILOCKS_PRIME;
            mds[i][j] = Goldilocks::new(denom).inv();
        }
    }
    mds
}

fn generate_round_constants(domain_tag: &[u8]) -> Vec<[Goldilocks; T]> {
    let total = TOTAL_ROUNDS * T * 8;
    let mut seed = domain_tag.to_vec();
    seed.extend_from_slice(b"_POSEIDON_RC");
    let mut hasher = Shake256::default();
    hasher.update(&seed);
    let mut reader = hasher.finalize_xof();
    let mut raw = vec![0u8; total];
    reader.read(&mut raw);
    let mut consts = Vec::with_capacity(TOTAL_ROUNDS);
    for r in 0..TOTAL_ROUNDS {
        let mut row = [Goldilocks::zero(); T];
        for i in 0..T {
            let offset = (r * T + i) * 8;
            let val = u64::from_be_bytes(raw[offset..offset + 8].try_into().unwrap()) % GOLDILOCKS_PRIME;
            row[i] = Goldilocks(val);
        }
        consts.push(row);
    }
    consts
}

fn mat_mul(state: &[Goldilocks; T], mds: &[[Goldilocks; T]; T]) -> [Goldilocks; T] {
    let mut result = [Goldilocks::zero(); T];
    for i in 0..T {
        let mut acc = Goldilocks::zero();
        for j in 0..T {
            acc = acc.add(mds[i][j].mul(state[j]));
        }
        result[i] = acc;
    }
    result
}

impl Poseidon {
    pub fn new(domain_tag: &[u8]) -> Self {
        Self {
            round_constants: generate_round_constants(domain_tag),
            mds: cauchy_mds(),
        }
    }

    pub fn permute(&self, state: &[Goldilocks; T]) -> [Goldilocks; T] {
        let mut st = *state;
        for r in 0..TOTAL_ROUNDS {
            for i in 0..T {
                st[i] = st[i].add(self.round_constants[r][i]);
            }
            if r < HALF_FULL || r >= HALF_FULL + PARTIAL_ROUNDS {
                for i in 0..T {
                    st[i] = st[i].pow(SBOX_ALPHA);
                }
            } else {
                st[0] = st[0].pow(SBOX_ALPHA);
            }
            st = mat_mul(&st, &self.mds);
        }
        st
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poseidon_deterministic() {
        let p = Poseidon::new(b"KLEINNER_KANTOR_5_V1");
        let state = [Goldilocks(0), Goldilocks(1), Goldilocks(2), Goldilocks(3)];
        let r1 = p.permute(&state);
        let r2 = p.permute(&state);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_poseidon_not_identity() {
        let p = Poseidon::new(b"KLEINNER_KANTOR_5_V1");
        let state = [Goldilocks::zero(); T];
        let result = p.permute(&state);
        assert!(result.iter().any(|x| x.0 != 0));
    }

    #[test]
    fn test_poseidon_avalanche() {
        let p = Poseidon::new(b"KLEINNER_KANTOR_5_V1");
        let s1 = [Goldilocks::zero(); T];
        let mut s2 = [Goldilocks::zero(); T];
        s2[0] = Goldilocks(1);
        let r1 = p.permute(&s1);
        let r2 = p.permute(&s2);
        assert_ne!(r1, r2);
    }
}
