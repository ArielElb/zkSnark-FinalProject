use num_bigint::BigUint;
use num_traits::One;

pub const K: usize = 4;
pub const NUM_BITS: usize = 32;

pub fn get_max_val() -> BigUint {
    BigUint::one() << NUM_BITS
}
