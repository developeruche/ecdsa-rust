/*!
This is a simple Elliptic curve library used for performing group binding operations
on a cyclic elliptic curve group.


Main Operations this lib would be handling would be;



-- Point Addition;

`
    R = P + Q
`

-- Point Doubling

`
    R = P + P = 2 * P
`


-- Scalar Multiplication

`
    R = d * P
`

where P, Q, R are points on a elliptic curve and, d is a scalar quantity.


======================

from studying elliptic curves, you would understand that elliptic curve can be defined algebraically by;

>> `y^2 = x^3 + ax + b mod p`

putting into consideration the constraint of this equation;

    1. `p` should be a prime number that is greater than `3`
    2. `4 a^3 + 27 b^2 != 0`

*/



use num_bigint::BigUint;
use crate::finite_fields;

///
/// This represents a point in the elliptic curve. The identity element is such
/// that:
///
///  - `A - A = I`
///  - `A + I = A`
///  - `I + I = 2 * I = I`
///
#[derive(Clone, PartialEq, Debug, )]
pub enum CurvePoint {
    Coordinate(BigUint, BigUint),
    Identity
}


#[derive(PartialEq, Debug)]
pub enum EllipticCurveError {
    InvalidPoint(CurvePoint),
    InvalidScalar(BigUint),
}



///
/// This represents an elliptic curve of the form
/// y^2 = x^3 + ax + b mod p
///
#[derive(PartialEq, Clone, Debug)]
pub struct EllipticCurve {
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint,
}



impl EllipticCurve {
    ///
    /// Perform a point addition: `C = A + B` where `A` and `B` are points which
    /// belong to the curve. Geometrically speaking, the point `C` is the
    /// x-reflection of the intersection of the lines that passes through `A`
    /// and `B` and intersects the curve.
    ///

    pub fn add(&self, a: &CurvePoint, b: &CurvePoint) -> Result<CurvePoint, EllipticCurveError> {
        // first thing is to make sure these points to be add are in the elliptic curve
        if !self.is_on_curve(a) {
            return Err(EllipticCurveError::InvalidPoint(a.clone()));
        }
        if !self.is_on_curve(b) {
            return Err(EllipticCurveError::InvalidPoint(b.clone()));
        }

        // the next edge case to test is when a == b, if a == b, the we would be using another formula stated in the double method
        if *a == *b {
            return self.double(a);
        }

        match (a, b) {
            (CurvePoint::Identity, _) => Ok(b.clone()),
            (_, CurvePoint::Identity) => Ok(a.clone()),
            (CurvePoint::Coordinate(x1, y1), CurvePoint::Coordinate(x2, y2)) => {
                let y1_plus_y2 = finite_fields::add(y1, y2, &self.p).unwrap();


                // checking if the sum is a point at infinity
                if x1 == x2 && y1_plus_y2 == BigUint::from(0u32) {
                    return Ok(CurvePoint::Identity);
                }


                // s = (y2 - y1) / (x2 - x1) mod p
                let numerator = finite_fields::subtract(y2, y1, &self.p).unwrap();
                let denominator = finite_fields::subtract(x2, x1, &self.p).unwrap();
                let s = finite_fields::divide(&numerator, &denominator, &self.p).unwrap();
                let (x3, y3) = self.compute_x3_y3(x1, y1, x2, &s);


                Ok(CurvePoint::Coordinate(x3, y3))
            }
        }
    }


    ///
    /// Perform a point doubling: `B = A + A = 2 * A` where `A` is a point in
    /// the curve. Geometrically speaking, the point `B` is the intersection of
    /// the tangent line over A that intersects the curve.
    ///
    pub fn double(&self, a: &CurvePoint) -> Result<CurvePoint, EllipticCurveError> {
        if !self.is_on_curve(a) {
            return Err(EllipticCurveError::InvalidPoint(a.clone()));
        }

        match a {
            CurvePoint::Identity => Ok(CurvePoint::Identity),
            CurvePoint::Coordinate(x1, y1) => {
                if *y1 == BigUint::from(0u32) {
                    return Ok(CurvePoint::Identity);
                }

                // s = (3 * x1^2 + a) / (2 * y1) mod p
                let numerator = x1.modpow(&BigUint::from(2u32), &self.p);
                let numerator =
                    finite_fields::multiplicate(&BigUint::from(3u32), &numerator, &self.p).unwrap();
                let numerator = finite_fields::add(&self.a, &numerator, &self.p).unwrap();
                let denominator = finite_fields::multiplicate(&BigUint::from(2u32), y1, &self.p).unwrap();
                let s = finite_fields::divide(&numerator, &denominator, &self.p).unwrap();
                let (x3, y3) = self.compute_x3_y3(x1, y1, x1, &s);

                Ok(CurvePoint::Coordinate(x3, y3))
            }
        }
    }



    ///
    /// computes the resulting point of the addition:
    ///  `C(x3,y3) = A(x1,y1) + B(x2,y2)`:
    ///
    /// `s` is given as input and should be computed differently depending on it
    /// is point doubling or point addition:
    ///
    /// - `B != A => s = (y2 - y1) / (x2 - x1) mod p`
    /// - `B == A => s = (3 * x1^2 + a) / (2 * y1) mod p`
    ///
    /// Result:
    ///
    /// - `x3 = s^2 - x1 - x2 mod p`
    /// - `y3 = s(x1 - x3) - y1 mod p`
    ///
    fn compute_x3_y3(
        &self,
        x1: &BigUint,
        y1: &BigUint,
        x2: &BigUint,
        s: &BigUint,
    ) -> (BigUint, BigUint) {
        let s2 = s.modpow(&BigUint::from(2u32), &self.p);
        let x3 = finite_fields::subtract(&s2, x1, &self.p).unwrap();
        let x3 = finite_fields::subtract(&x3, x2, &self.p).unwrap();

        let y3 = finite_fields::subtract(x1, &x3, &self.p).unwrap();
        let y3 = finite_fields::multiplicate(s, &y3, &self.p).unwrap();
        let y3 = finite_fields::subtract(&y3, y1, &self.p).unwrap();

        (x3, y3)
    }


    ///
    /// Perform a scalar multiplication of a point: `B = d * A` where `A` is a
    /// point in the curve and `d > 0` is a positive scalar of any value.
    ///
    /// It uses the addition/doubling algorithm
    ///
    /// ```text
    ///  T = A
    ///  for i in [(bits of d)-1), 0]
    ///       T = 2 * T
    ///       if bit i of d == 1
    ///           T = T + A
    /// ```
    ///
    pub fn scalar_mul(&self, a: &CurvePoint, d: &BigUint) -> Result<CurvePoint, EllipticCurveError> {
        if *d == BigUint::from(0u32) {
            return Err(EllipticCurveError::InvalidScalar(d.clone()));
        }

        let mut t = a.clone();
        for i in (0..(d.bits() - 1)).rev() {
            t = self.double(&t)?;
            if d.bit(i) {
                t = self.add(&t, a)?;
            }
        }
        Ok(t)
    }


    ///
    /// Checks if a point A = (x,y) belongs to the elliptic curve:
    ///
    /// if `y^2 == x^3 + a * x + b mod p` then returns `true`, if not, returns
    /// `false`.
    ///
    pub fn is_on_curve(&self, a: &CurvePoint) -> bool {
        match a {
            CurvePoint::Coordinate(x, y) => {
                let y2 = y.modpow(&BigUint::from(2u32), &self.p);
                let x3 = x.modpow(&BigUint::from(3u32), &self.p);
                let a_x = finite_fields::multiplicate(&self.a, x, &self.p).unwrap();
                let x3_plus_ax = finite_fields::add(&x3, &a_x, &self.p).unwrap();

                y2 == finite_fields::add(&x3_plus_ax, &self.b, &self.p).unwrap()
            }
            CurvePoint::Identity => true, // the identity element is always on the curve, there no need try another
        }
    }
}