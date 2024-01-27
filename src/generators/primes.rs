use num_bigint::BigUint;

/// Generates all prime numbers up to a given maximum value.
///
/// This function uses the Sieve of Eratosthenes algorithm to efficiently generate all prime numbers less than
/// or equal to the specified maximum value. It is useful for tasks that require a list of small prime numbers.
///
/// # Arguments
///
/// * `maximum` - A `u64` representing the maximum value up to which prime numbers are to be generated.
///
/// # Returns
///
/// A vector of `BigUint` containing all prime numbers less than or equal to `maximum`.
/// Returns an empty vector if `maximum` is less than 2.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use large_primes::get_max_primes;
/// 
/// let max_value = 10;
/// let primes = get_max_primes(max_value);
/// assert_eq!(primes, vec![BigUint::from(2u32), BigUint::from(3u32), BigUint::from(5u32), BigUint::from(7u32)]);
/// ```
pub fn get_max_primes(maximum: u64) -> Vec<BigUint> {
    if maximum < 2 {
        return Vec::<BigUint>::new();
    }
    let mut primes: Vec<BigUint> = Vec::new();
    let mut sieve = vec![true; (maximum+1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..maximum + 1 {
        if sieve[i as usize] {
            primes.push(BigUint::from(i));
            let mut j = i * i;
            while j < maximum + 1 {
                sieve[j as usize] = false;
                j += i;
            }
        }
    }
    return primes;
}

#[cfg(test)]
mod tests {
    #[test]
    fn edge_cases() {
        // Test case 0: Empty
        assert_eq!(super::get_max_primes(0), Vec::<super::BigUint>::new());

        // Test case 1: Empty
        assert_eq!(super::get_max_primes(1), Vec::<super::BigUint>::new());

        // Test case 2: [2]
        assert_eq!(super::get_max_primes(2), vec![super::BigUint::from(2u32)]);

        // Test case 3: [2, 3]
        assert_eq!(
            super::get_max_primes(3),
            vec![super::BigUint::from(2u32), super::BigUint::from(3u32)]
        );

        // Test case 4: [2, 3]
        assert_eq!(
            super::get_max_primes(4),
            vec![super::BigUint::from(2u32), super::BigUint::from(3u32)]
        );
    }

    #[test]
    fn large_test() {
        // Generate Huge list of prime numbers
        let primes = super::get_max_primes(10000000);
        assert_eq!(primes.len(), 664579);

        assert_eq!(primes[0], super::BigUint::from(2u32));
        assert_eq!(primes[9], super::BigUint::from(29u32));
        assert_eq!(primes[99], super::BigUint::from(541u32));
        assert_eq!(primes[999], super::BigUint::from(7919u32));
        assert_eq!(primes[9999], super::BigUint::from(104729u32));
        assert_eq!(primes[99999], super::BigUint::from(1299709u32));
        assert_eq!(primes[599999], super::BigUint::from(8960453u32));
    }
}
