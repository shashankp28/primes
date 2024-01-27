use num_bigint::BigUint;
use num_traits::Zero;

pub fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    // Euclidean algorithm
    let mut a = a.clone();
    let mut b = b.clone();

    let zero = BigUint::zero();

    while b > zero {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }

    return a;
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::{ One, Zero };

    use super::*;

    #[test]
    fn edge_cases() {
        // Test case 0: 0
        assert_eq!(gcd(&BigUint::zero(), &BigUint::zero()), BigUint::zero());

        // Test case 1: 1
        assert_eq!(gcd(&BigUint::one(), &BigUint::zero()), BigUint::one());

        // Test case 2: 1
        assert_eq!(gcd(&BigUint::zero(), &BigUint::one()), BigUint::one());

        // Test case 3: 2
        assert_eq!(gcd(&BigUint::from(2u32), &BigUint::from(2u32)), BigUint::from(2u32));

        // Test case 4: 1
        assert_eq!(gcd(&BigUint::from(2u32), &BigUint::from(3u32)), BigUint::one());

        // Test case 5: 2
        assert_eq!(gcd(&BigUint::from(2u32), &BigUint::from(4u32)), BigUint::from(2u32));

        // Test case 6: 1
        assert_eq!(gcd(&BigUint::from(3u32), &BigUint::from(5u32)), BigUint::one());

        // Test case 7: 3
        assert_eq!(gcd(&BigUint::from(3u32), &BigUint::from(6u32)), BigUint::from(3u32));

        // Test case 8: 1
        assert_eq!(gcd(&BigUint::from(4u32), &BigUint::from(7u32)), BigUint::one());

        // Test case 9: 4
        assert_eq!(gcd(&BigUint::from(4u32), &BigUint::from(8u32)), BigUint::from(4u32));

        // Test case 10: 1
        assert_eq!(gcd(&BigUint::from(5u32), &BigUint::from(9u32)), BigUint::one());

        // Test case 11: 5
        assert_eq!(gcd(&BigUint::from(5u32), &BigUint::from(10u32)), BigUint::from(5u32));
    }

    #[test]
    fn general_cases() {
        // Big Numbers 6-8 digits

        assert_eq!(
            gcd(&BigUint::from(123456u32), &BigUint::from(123456u32)),
            BigUint::from(123456u32)
        );
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123457u32)), BigUint::one());
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123458u32)), BigUint::from(2u32));
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123459u32)), BigUint::from(3u32));
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123460u32)), BigUint::from(4u32));
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123461u32)), BigUint::one());
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123462u32)), BigUint::from(6u32));
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123463u32)), BigUint::one());
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123464u32)), BigUint::from(8u32));
        assert_eq!(gcd(&BigUint::from(123456u32), &BigUint::from(123465u32)), BigUint::from(3u32));
    }
}
