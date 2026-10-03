pub const GOLDILOCKS_PRIME: u64 = 18_446_744_069_414_584_321;

pub fn mul_mod(a: u64, b: u64) -> u64 {
    let wide = (a as u128) * (b as u128);
    let reduced = wide % (GOLDILOCKS_PRIME as u128);
    reduced as u64
}

pub fn add_mod(a: u64, b: u64) -> u64 {
    let (sum, carry) = a.overflowing_add(b);
    if carry || sum >= GOLDILOCKS_PRIME {
        sum.wrapping_sub(GOLDILOCKS_PRIME)
    } else {
        sum
    }
}

pub fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a.wrapping_sub(b).wrapping_add(GOLDILOCKS_PRIME)
    }
}

pub fn pow_mod(base: u64, exp: u64) -> u64 {
    let mut result = 1u64;
    let mut b = base;
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            result = mul_mod(result, b);
        }
        b = mul_mod(b, b);
        e >>= 1;
    }
    result
}

pub fn inv_mod(a: u64) -> u64 {
    pow_mod(a, GOLDILOCKS_PRIME - 2)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Goldilocks(pub u64);

impl Goldilocks {
    pub fn new(val: u64) -> Self {
        Self(val % GOLDILOCKS_PRIME)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn add(self, other: Self) -> Self {
        Self(add_mod(self.0, other.0))
    }

    pub fn sub(self, other: Self) -> Self {
        Self(sub_mod(self.0, other.0))
    }

    pub fn mul(self, other: Self) -> Self {
        Self(mul_mod(self.0, other.0))
    }

    pub fn pow(self, exp: u64) -> Self {
        Self(pow_mod(self.0, exp))
    }

    pub fn inv(self) -> Self {
        Self(inv_mod(self.0))
    }

    pub fn to_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let val = u64::from_be_bytes(bytes[..8].try_into().unwrap());
        Self(val % GOLDILOCKS_PRIME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_associative() {
        let a = Goldilocks::new(123456789);
        let b = Goldilocks::new(987654321);
        let c = Goldilocks::new(555555);
        assert_eq!((a.add(b)).add(c), a.add(b.add(c)));
    }

    #[test]
    fn test_mul_commutative() {
        let a = Goldilocks::new(123456789);
        let b = Goldilocks::new(987654321);
        assert_eq!(a.mul(b), b.mul(a));
    }

    #[test]
    fn test_inverse() {
        let a = Goldilocks::new(42);
        assert_eq!(a.mul(a.inv()).0, 1);
    }

    #[test]
    fn test_from_to_bytes() {
        let a = Goldilocks::new(0xdeadbeefcafebabe);
        let bytes = a.to_bytes();
        let b = Goldilocks::from_bytes(&bytes);
        assert_eq!(a, b);
    }
}
