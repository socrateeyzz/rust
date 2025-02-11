use std::ops::Add;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BigInt {
    integer: [u64; 4]
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, _other: BigInt) -> BigInt {
        BigInt {
            integer: [0, 0, 0, 0]
        }
    }
}

impl Default for BigInt {
    fn default() -> Self {
        BigInt {
            integer: [0, 0, 0, 0]
        }
    }
}
