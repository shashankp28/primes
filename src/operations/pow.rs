use num_bigint::BigUint;
use num_traits::{ One, Zero };

pub fn pow(base: &BigUint, exp: &BigUint) -> BigUint {
    // Modular exponentiation
    let mut result = BigUint::one();
    let mut current_base = base.clone();

    let zero = BigUint::zero();
    let one = BigUint::one();

    let two = BigUint::from(2u32);

    let mut power = exp.clone();

    while &power > &zero {
        if &power % &two == one {
            result = &result * &current_base;
        }

        let shifted_power: BigUint = &power >> 1;
        power = shifted_power.clone();
        current_base = &current_base * &current_base;
    }

    result
}

pub fn pow_mod(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    // Modular exponentiation
    let mut result = BigUint::one();
    let mut base = base % modulus;

    let zero = BigUint::zero();
    let one = BigUint::one();

    let two = BigUint::from(2u32);

    let mut power = exp.clone();

    while &power > &zero {
        if &power % &two == one {
            result = (&result * &base) % modulus;
        }

        let shifted_power: BigUint = &power >> 1;
        power = shifted_power.clone();
        base = (&base * &base) % modulus;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_power() {
        let base = BigUint::from(2u32);
        let exp = BigUint::zero();

        assert_eq!(pow(&base, &exp), BigUint::from(1u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::from(10u32);

        assert_eq!(pow(&base, &exp), BigUint::from(1024u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::from(100u32);

        assert_eq!(
            pow(&base, &exp),
            BigUint::parse_bytes(b"1267650600228229401496703205376", 10).unwrap()
        );
    }

    #[test]
    fn base_cases() {
        let base = BigUint::from(2u32);
        let exp = BigUint::from(2u32);
        let modulus = BigUint::from(3u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(1u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::zero();
        let modulus = BigUint::from(3u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(1u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::from(1u32);
        let modulus = BigUint::from(2u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(0u32));
    }

    #[test]
    fn big_cases() {
        let base = BigUint::from(2u32);
        let exp = BigUint::from(10u32);
        let modulus = BigUint::from(100u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(24u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::from(100u32);
        let modulus = BigUint::from(1000u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(376u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::from(1000u32);
        let modulus = BigUint::from(10000u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(9376u32));

        let base = BigUint::from(2u32);
        let exp = BigUint::from(10000u32);
        let modulus = BigUint::from(100000u32);

        assert_eq!(pow_mod(&base, &exp, &modulus), BigUint::from(9376u32));
    }
}
