use blake3;

pub struct K5MemoryHard {
    pub mem_blocks: usize,
    pub time_cost: u32,
}

impl K5MemoryHard {
    pub fn new(mem_kib: usize, time_cost: u32) -> Self {
        Self {
            mem_blocks: (mem_kib * 64).max(16),
            time_cost: time_cost.max(1),
        }
    }

    pub fn process(&self, data: &[u8]) -> [u8; 32] {
        let state = blake3::hash(data);
        let n = self.mem_blocks;
        let mut buf: Vec<[u8; 16]> = Vec::with_capacity(n);
        for i in 0..n {
            let mut seed = state.as_bytes().to_vec();
            seed.extend_from_slice(&(i as u64).to_be_bytes());
            let h = blake3::hash(&seed);
            buf.push(h.as_bytes()[..16].try_into().unwrap());
        }
        for t in 0..self.time_cost {
            for i in 0..n {
                let prev = buf[(i + n - 1) % n];
                let dep_idx = u64::from_be_bytes(buf[i][..8].try_into().unwrap()) as usize % n;
                let dep = buf[dep_idx];
                let mut mix = buf[i].to_vec();
                mix.extend_from_slice(&prev);
                mix.extend_from_slice(&dep);
                buf[i] = blake3::hash(&mix).as_bytes()[..16].try_into().unwrap();
            }
            for i in 0..n {
                let mut mix = buf[i].to_vec();
                mix.extend_from_slice(&buf[(i + t as usize) % n][..8]);
                buf[i] = blake3::hash(&mix).as_bytes()[..16].try_into().unwrap();
            }
        }
        let mut xored = [0u8; 32];
        for b in &buf {
            for (j, &byte) in b.iter().enumerate() {
                xored[j % 32] ^= byte;
            }
            for (j, &byte) in b.iter().enumerate() {
                xored[(j + 16) % 32] ^= byte;
            }
        }
        let mut final_input = xored.to_vec();
        final_input.extend_from_slice(state.as_bytes());
        let result = blake3::hash(&final_input);
        *result.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_hard_deterministic() {
        let mh = K5MemoryHard::new(64, 1);
        let r1 = mh.process(b"test");
        let r2 = mh.process(b"test");
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_memory_hard_differs() {
        let mh = K5MemoryHard::new(64, 1);
        let r1 = mh.process(b"hello");
        let r2 = mh.process(b"world");
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_memory_hard_diff_time() {
        let r1 = K5MemoryHard::new(64, 1).process(b"test");
        let r3 = K5MemoryHard::new(64, 3).process(b"test");
        assert_ne!(r1, r3);
    }
}
