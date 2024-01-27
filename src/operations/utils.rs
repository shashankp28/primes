use num_bigint::BigUint;
use num_traits::One;
use num_traits::Zero;

/// Counts the number of trailing zeros in the binary representation of a `BigUint`.
///
/// This function calculates the number of consecutive zero bits starting from the least significant bit
/// (rightmost bit) of the given `BigUint` number. It is particularly useful in algorithms that require
/// analysis of a number's binary structure, such as in certain primality tests.
///
/// # Arguments
///
/// * `num` - A reference to a `BigUint` representing the number whose trailing zeros are to be counted.
///
/// # Returns
///
/// The number of trailing zeros in the binary representation of `num`.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// let number = BigUint::from(8u32); // Binary: 1000
/// let trailing_zeros = get_trailing_zeros(&number);
/// assert_eq!(trailing_zeros, BigUint::from(3u32)); // Three trailing zeros
/// ```
pub fn get_trailing_zeros(num: &BigUint) -> BigUint {
    let mut trailing_zeros = BigUint::zero();
    let mut num_copy = num.clone(); // Clone the number to work with

    while (&num_copy & &BigUint::one()) == BigUint::zero() {
        num_copy = num_copy >> 1u32; // Right shift to divide by 2
        trailing_zeros += BigUint::one();
    }

    trailing_zeros
}
