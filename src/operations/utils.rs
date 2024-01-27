use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

pub fn get_trailing_zeros(num: &BigUint) -> BigUint {
    let mut trailing_zeros = BigUint::zero();
    let mut num_copy = num.clone(); // Clone the number to work with

    while (&num_copy & &BigUint::one()) == BigUint::zero() {
        num_copy = num_copy >> 1u32; // Right shift to divide by 2
        trailing_zeros += BigUint::one();
    }

    trailing_zeros
}
