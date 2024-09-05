use num_bigint::BigUint;
use num_traits::One;

pub const K: usize = 2;
pub const NUM_BITS: usize = 20;

pub fn get_max_val()->BigUint {
    BigUint::one() << NUM_BITS
}
