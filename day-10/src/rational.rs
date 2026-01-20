use std::cmp::{Ord, Ordering, PartialOrd};
use std::convert::{From, TryFrom};
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Rational {
    pub numerator: isize,
    pub denominator: isize,
}

impl From<isize> for Rational {
    fn from(num: isize) -> Self {
        Self::new(num, 1)
    }
}

#[derive(Debug)]
pub struct ConversionError;

impl TryFrom<Rational> for isize {
    type Error = ConversionError;
    fn try_from(value: Rational) -> Result<Self, Self::Error> {
        if value.denominator == 1 {
            Ok(value.numerator)
        } else {
            Err(ConversionError)
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Rational {
    pub fn new(numerator: isize, denominator: isize) -> Self {
        assert!(denominator != 0);

        if numerator == 0 {
            return Self {
                numerator: 0,
                denominator: 1,
            };
        }

        let gcd = gcd(numerator.abs(), denominator.abs());
        let reduced_n = numerator / gcd;
        let reduced_d = denominator / gcd;
        let sign = reduced_n.signum() / reduced_d.signum();

        Self {
            numerator: reduced_n.abs() * sign,
            denominator: reduced_d.abs(),
        }
    }

    pub fn abs(&self) -> Self {
        if self.numerator < 0 {
            *self * Self::from(-1)
        } else {
            *self
        }
    }

    pub fn floor(&self) -> isize {
        (self.numerator as f64 / self.denominator as f64).floor() as isize
    }

    pub fn ceil(&self) -> isize {
        (self.numerator as f64 / self.denominator as f64).ceil() as isize
    }

    pub fn is_non_negative_integer(&self) -> bool {
        self.denominator == 1 && self.numerator >= 0
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.numerator * other.denominator + self.denominator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.numerator * other.denominator - self.denominator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * Self::from(-1)
    }
}

impl Sum for Rational {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|result, num| result + num).unwrap_or(0.into())
    }
}

fn gcd(x: isize, y: isize) -> isize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization() {
        assert_eq!(Rational::new(0, -200), Rational::new(0, 1));
        assert_eq!(Rational::new(100, 200), Rational::new(1, 2));
        assert_eq!(Rational::new(10, -15), Rational::new(-2, 3));
        assert_eq!(Rational::new(-4, -16), Rational::new(1, 4));
    }

    #[test]
    fn into() {
        let two: Rational = 2.into();
        let three = Rational::from(3);
        assert_eq!(two, Rational::new(2, 1));
        assert_eq!(three, Rational::new(3, 1));
    }

    #[test]
    fn add() {
        let a = Rational::new(1, 3);
        let b = Rational::new(1, 4);
        assert_eq!(a + b, Rational::new(7, 12));
    }

    #[test]
    fn add_assign() {
        let mut a = Rational::new(1, 3);
        let b = Rational::new(1, 4);
        a += b;
        assert_eq!(a, Rational::new(7, 12));
    }

    #[test]
    fn sub() {
        let a = Rational::new(1, 3);
        let b = Rational::new(4, 3);
        assert_eq!(a - b, Rational::new(-1, 1));
    }

    #[test]
    fn sub_assign() {
        let mut a = Rational::new(1, 3);
        let b = Rational::new(4, 3);
        a -= b;
        assert_eq!(a, Rational::new(-1, 1));
    }

    #[test]
    fn mul() {
        let a = Rational::new(1, 2);
        let b = Rational::new(1, 2);
        assert_eq!(a * b, Rational::new(1, 4));
    }

    #[test]
    fn mul_assign() {
        let mut a = Rational::new(1, 2);
        let b = Rational::new(1, 2);
        a *= b;
        assert_eq!(a, Rational::new(1, 4));
    }

    #[test]
    fn div() {
        let a = Rational::new(1, 2);
        let b = Rational::new(1, 2);
        assert_eq!(a / b, Rational::new(1, 1));
    }

    #[test]
    fn div_assign() {
        let mut a = Rational::new(1, 2);
        let b = Rational::new(1, 2);
        a /= b;
        assert_eq!(a, 1.into());
    }

    #[test]
    fn neg() {
        let a = Rational::new(-7, 3);
        assert_eq!(-a, Rational::new(7, 3));
    }

    #[test]
    fn cmp() {
        let a = Rational::new(1, 2);
        let b = Rational::new(1, 3);
        assert!(a > b);
        assert!(b < a);
    }

    #[test]
    fn floor_and_ceil() {
        assert_eq!(Rational::new(1, 3).floor(), 0);
        assert_eq!(Rational::new(1, 3).ceil(), 1);
    }
}
