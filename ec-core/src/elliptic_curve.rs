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

        todo!()
    }


    //
    // Checks if a point A = (x,y) belongs to the elliptic curve:
    //
    // if `y^2 = x^3 + a * x + b mod p` then returns `true`, if not, returns
    // `false`.
    //
    // pub fn is_on_curve(&self, a: &Point) -> bool {
    //     match a {
    //         CurvePoint::Coordinate(x, y) => {
    //             let y2 = y.modpow(&BigUint::from(2u32), &self.p);
    //             let x3 = x.modpow(&BigUint::from(3u32), &self.p);
    //             let ax = FiniteField::mult(&self.a, x, &self.p).unwrap();
    //             let x3plusax = FiniteField::add(&x3, &ax, &self.p).unwrap();
    //
    //             y2 == FiniteField::add(&x3plusax, &self.b, &self.p).unwrap()
    //         }
    //         Point::Identity => true,
    //     }
    // }
}