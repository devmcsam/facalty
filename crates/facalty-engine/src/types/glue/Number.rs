#![allow(clippy::cast_precision_loss)]
// converting int to float, this actually is more precise than having integer division truncate.
#![allow(clippy::cast_possible_truncation)]
// i64 to u32, done in pow function
#![allow(clippy::cast_sign_loss)]
// also for pow function, a check is made that the exponent is positive
use num_rational::Rational64;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Number {
    Int(i64),
    Rational(Rational64),
    Float(f64),
}

impl Number {
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::Int(_))
    }
    pub const fn is_rational(&self) -> bool {
        matches!(self, Self::Rational(_))
    }
    pub const fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }
    pub fn is_zero(&self) -> bool {
        match self {
            Self::Int(n) => *n == 0,
            Self::Rational(r) => r == &Rational64::from_integer(0),
            Self::Float(f) => *f == 0.0,
        }
    }
    pub fn is_one(&self) -> bool {
        match self {
            Self::Int(n) => *n == 1,
            Self::Rational(r) => r == &Rational64::from_integer(1),
            Self::Float(f) => (*f - 1.0).abs() < f64::EPSILON,
        }
    }

    /// Promote to Rational (Int → Rational, others unchanged)
    pub fn to_rational(self) -> Self {
        match self {
            Self::Int(n) => Self::Rational(Rational64::from_integer(n)),
            other => other,
        }
    }

    /// Promote to f64 (any variant → Float)
    pub fn to_float(self) -> Self {
        match self {
            Self::Int(n) => Self::Float(n as f64),
            Self::Rational(r) => Self::Float(*r.numer() as f64 / *r.denom() as f64),
            Self::Float(_) => self,
        }
    }

    /// Extract the f64 value (useful internally)
    fn as_f64(self) -> f64 {
        match self {
            Self::Int(n) => n as f64,
            Self::Rational(r) => *r.numer() as f64 / *r.denom() as f64,
            Self::Float(f) => f,
        }
    }

    /// Extract as Rational64 (only valid for Int/Rational)
    fn as_rational(self) -> Rational64 {
        match self {
            Self::Int(n) => Rational64::from_integer(n),
            Self::Rational(r) => r,
            Self::Float(_) => unreachable!("as_rational called on Float"),
        }
    }

    /// Integer exponentiation helper. Negative exponents promote to Rational.
    pub fn pow(self, exp: Self) -> Self {
        match (self, exp) {
            // Both ints — stay int if exp >= 0, else go rational
            (Self::Int(base), Self::Int(e)) => {
                if e >= 0 {
                    Self::Int(base.pow(e as u32))
                } else {
                    let r = Rational64::new(1, base.pow((-e) as u32));
                    Self::Rational(r)
                }
            }
            // If either is float, go float
            (a, b) if a.is_float() || b.is_float() => Self::Float(a.as_f64().powf(b.as_f64())),
            // Rational base with integer exponent — stay rational
            (Self::Rational(r), Self::Int(e)) => Self::Rational(r.pow(e as i32)),
            // Rational exponent → must go float (no exact representation)
            (a, b) => Self::Float(a.as_f64().powf(b.as_f64())),
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────

/// Applies an op depending on whether either operand is Float, Rational, or both Int.
/// `int_op` is tried first for two Ints and may return None to signal promotion
/// (e.g. division that doesn't divide evenly — though here we always promote div).
fn promote_and_apply(
    a: Number,
    b: Number,
    int_op: impl FnOnce(i64, i64) -> Option<Number>,
    rat_op: impl FnOnce(Rational64, Rational64) -> Number,
    float_op: impl FnOnce(f64, f64) -> Number,
) -> Number {
    match (a, b) {
        // If either is float → float
        (Number::Float(fa), other) => float_op(fa, other.as_f64()),
        (other, Number::Float(fb)) => float_op(other.as_f64(), fb),

        // Both ints → try int, fall back to rational
        (Number::Int(ia), Number::Int(ib)) => int_op(ia, ib)
            .unwrap_or_else(|| rat_op(Rational64::from_integer(ia), Rational64::from_integer(ib))),

        // At least one rational (and no floats) → rational
        (ra, rb) => rat_op(ra.as_rational(), rb.as_rational()),
    }
}

// ── Add ─────────────────────────────────────────────────────────

impl Add for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        promote_and_apply(
            self,
            rhs,
            |a, b| a.checked_add(b).map(Self::Int),
            |a, b| Self::Rational(a + b),
            |a, b| Self::Float(a + b),
        )
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

// ── Sub ─────────────────────────────────────────────────────────

impl Sub for Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        promote_and_apply(
            self,
            rhs,
            |a, b| a.checked_sub(b).map(Self::Int),
            |a, b| Self::Rational(a - b),
            |a, b| Self::Float(a - b),
        )
    }
}

impl SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// ── Mul ─────────────────────────────────────────────────────────

impl Mul for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        promote_and_apply(
            self,
            rhs,
            |a, b| a.checked_mul(b).map(Self::Int),
            |a, b| Self::Rational(a * b),
            |a, b| Self::Float(a * b),
        )
    }
}

impl MulAssign for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// ── Div (always promotes Int→Rational to stay exact) ────────────

impl Div for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        promote_and_apply(
            self,
            rhs,
            |_a, _b| {
                // Always promote to Rational for exactness
                None::<Self>
                // The None triggers the rat_op fallback
            },
            |a, b| Self::Rational(a / b),
            |a, b| Self::Float(a / b),
        )
    }
}

impl DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// ── Rem ─────────────────────────────────────────────────────────

impl Rem for Number {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        promote_and_apply(
            self,
            rhs,
            |a, b| a.checked_rem(b).map(Self::Int),
            |a, b| Self::Rational(a % b),
            |a, b| Self::Float(a % b),
        )
    }
}

impl RemAssign for Number {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs;
    }
}

// ── Neg ─────────────────────────────────────────────────────────

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::Int(n) => Self::Int(-n),
            Self::Rational(r) => Self::Rational(-r),
            Self::Float(f) => Self::Float(-f),
        }
    }
}

// ── Display ─────────────────────────────────────────────────────

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{n}"),
            Self::Rational(r) => write!(f, "{}/{}", r.numer(), r.denom()),
            Self::Float(v) => write!(f, "{v}"),
        }
    }
}
