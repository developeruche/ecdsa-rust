///
/// A library which implements the bottom layer finite field group operations needed to
/// operate with the coordinates of the elliptic curve group.
///
use num_bigint::BigUint;




#[derive(Debug, PartialEq)]
pub enum FiniteFieldError {
    InvalidArgument(String),
    InvalidResult(String),
}


///
/// Adds to elements in the set (this would also return a point in the curve)
///
/// `a + b = a mod p`
///

pub fn add(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;

    Ok((a + b).modpow(&BigUint::from(1u32), p))
}



///
/// Multiplies to elements in the set
///
/// `a * b = a mod p`
///
pub fn multiplicate(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;

    Ok((a * b).modpow(&BigUint::from(1u32), p))
}


///
/// Finds the additive inverse of an element in the set:
///
/// `a + (-a) = 0 mod p`
///
pub fn inverse_add(a: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check_single_point(a,p)?;
    if *a == BigUint::from(0u32) {
        return Ok(a.clone());
    }

    Ok(p - a)
}


///
/// Subtract two elements in the set:
///
/// `a - b = a + (-b) = a mod p`
///
pub fn subtract(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;
    let b_inverse = inverse_add(b, p)?;

    add(a, &b_inverse, p)
}


///
/// Finds the multiplicative inverse of an element in the set if p is a
/// prime number using Fermat's Little Theorem:
///
/// `a^(-1) mod p = a^(p-2) mod p`
///
/// Such that:
/// `a * a^(-1) = 1 mod p`
///
pub fn inverse_multiplicate_prime(a: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check_single_point(a, p)?;
    Ok(a.modpow(&(p - BigUint::from(2u32)), p))
}



///
/// This function check if `a  < b`; if a is b, function would return true
///
pub fn check_is_less_than(a: &BigUint, b: &BigUint)  -> bool {
    if a < b {
        true
    } else {
        false
    }
}

///
/// When adding elliptic curves point, it is important that the points to be added id not
/// greater than the operating mob p of the group.
pub fn params_to_mod_check(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<(), FiniteFieldError> {
    let params_check = check_is_less_than(a, p) && check_is_less_than(b, p);
    if !params_check {
        return Err(FiniteFieldError::InvalidArgument(format!("a and b has to be greater than p")))
    }

    Ok(())
}

pub fn params_to_mod_check_single_point(a: &BigUint, p: &BigUint) -> Result<(), FiniteFieldError> {
    let params_check = check_is_less_than(a, p);
    if !params_check {
        return Err(FiniteFieldError::InvalidArgument(format!("a and b has to be greater than p")))
    }

    Ok(())
}


///
/// Divides two elements in the set:
///
/// `a / b = a * b^(-1) = a mod p`
///
pub fn divide(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;
    let b_inverse = inverse_multiplicate_prime(b, p)?;

    multiplicate(a, &b_inverse, p)
}

