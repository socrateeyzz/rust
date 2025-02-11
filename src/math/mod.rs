use std::ops::Add;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct BigInt {
    integer: [u64; 4]
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        return BigInt {
            integer: [0, 0, 0, 0]
        }
    }
}
