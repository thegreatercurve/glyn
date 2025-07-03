use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use crate::value::string::JSString;

/// 6.1.6.1 The Number Type
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum JSNumber {
    Float(f64),
    Int(i32),
    UInt(u32),
}

impl JSNumber {
    fn as_f64(&self) -> f64 {
        match self {
            JSNumber::Float(f) => *f,
            JSNumber::Int(i) => *i as f64,
            JSNumber::UInt(u) => *u as f64,
        }
    }

    fn as_i32(&self) -> i32 {
        match self {
            JSNumber::Float(f) => *f as i32,
            JSNumber::Int(i) => *i,
            JSNumber::UInt(u) => *u as i32,
        }
    }

    fn as_u32(&self) -> u32 {
        match self {
            JSNumber::Float(f) => *f as u32,
            JSNumber::Int(i) => *i as u32,
            JSNumber::UInt(u) => *u,
        }
    }

    pub(crate) fn nan() -> Self {
        JSNumber::Float(f64::NAN)
    }

    fn is_int(&self) -> bool {
        matches!(self, JSNumber::Int(_))
    }

    pub(crate) fn is_zero(&self) -> bool {
        match self {
            JSNumber::Float(f) => *f == 0.0 || self.is_pos_zero() || self.is_neg_zero(),
            JSNumber::Int(i) => *i == 0,
            JSNumber::UInt(u) => *u == 0,
        }
    }

    fn is_pos_zero(&self) -> bool {
        match self {
            JSNumber::Float(f) => *f == 0.0 && f.is_sign_positive(),
            JSNumber::Int(i) => *i == 0,
            JSNumber::UInt(u) => *u == 0,
        }
    }

    fn is_neg_zero(&self) -> bool {
        match self {
            JSNumber::Float(f) => *f == 0.0 && f.is_sign_negative(),
            JSNumber::Int(i) => *i == 0,
            JSNumber::UInt(u) => *u == 0,
        }
    }

    pub(crate) fn is_nan(&self) -> bool {
        match self {
            JSNumber::Float(f) => f.is_nan(),
            JSNumber::Int(_) => false,
            JSNumber::UInt(_) => false,
        }
    }

    pub(crate) fn is_finite(&self) -> bool {
        match self {
            JSNumber::Float(f) => f.is_finite(),
            JSNumber::Int(_) => true,
            JSNumber::UInt(_) => true,
        }
    }

    fn is_infinite(&self) -> bool {
        match self {
            JSNumber::Float(f) => f.is_infinite(),
            JSNumber::Int(_) => false,
            JSNumber::UInt(_) => false,
        }
    }

    pub(crate) fn is_pos_infinite(&self) -> bool {
        match self {
            JSNumber::Float(f) => f.is_infinite() && *f > 0.0,
            JSNumber::Int(_) => false,
            JSNumber::UInt(_) => false,
        }
    }

    pub(crate) fn is_neg_infinite(&self) -> bool {
        match self {
            JSNumber::Float(f) => f.is_infinite() && *f < 0.0,
            JSNumber::Int(_) => false,
            JSNumber::UInt(_) => false,
        }
    }

    pub(crate) fn truncate(&self) -> Self {
        match self {
            JSNumber::Float(f) => JSNumber::Float(f.trunc()),
            JSNumber::Int(i) => JSNumber::Int(*i as i32),
            JSNumber::UInt(u) => JSNumber::UInt(*u as u32),
        }
    }
}

impl JSNumber {
    /// 6.1.6.1.3 Number::exponentiate ( base, exponent )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-exponentiate
    fn exponentiate(self, other: Self) -> Self {
        match self {
            JSNumber::Float(f) => JSNumber::Float(f.powf(other.as_f64())),
            JSNumber::Int(i) => JSNumber::Int(i.pow(other.as_u32())),
            JSNumber::UInt(u) => JSNumber::UInt(u.pow(other.as_u32())),
        }
    }

    /// 6.1.6.1.11 Number::unsignedRightShift ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-unsignedRightShift
    fn ushr(self, other: Self) -> Self {
        JSNumber::UInt(self.as_u32() >> other.as_u32())
    }

    /// 6.1.6.1.13 Number::equal ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-equal
    fn equal(self, y: Self) -> bool {
        // 1. If x is NaN, return false.
        // 2. If y is NaN, return false.
        if self.is_nan() || y.is_nan() {
            return false;
        };

        // 3. If x is y, return true.
        // 4. If x is +0𝔽 and y is -0𝔽, return true.
        // 5. If x is -0𝔽 and y is +0𝔽, return true.
        if self.as_f64() == y.as_f64()
            || (self.is_pos_zero() && y.is_neg_zero())
            || (self.is_neg_zero() && y.is_pos_zero())
        {
            return true;
        }

        // 6. Return false.
        false
    }

    /// 6.1.6.1.14 Number::sameValue ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-samevalue
    pub(crate) fn same_value(&self, y: &Self) -> bool {
        // 1. 1. If x is NaN and y is NaN, return true.
        if self.is_nan() || y.is_nan() {
            return true;
        }

        // 2. If x is +0𝔽 and y is -0𝔽, return false.
        // 3. If x is -0𝔽 and y is +0𝔽, return false.
        if (self.is_pos_zero() && y.is_neg_zero()) || (self.is_neg_zero() && y.is_pos_zero()) {
            return false;
        }

        // 4. If x is y, return true.
        self == y
    }

    /// 6.1.6.1.20 Number::toString ( x, radix )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-tostring
    pub(crate) fn to_string(&self, radix: u32) -> JSString {
        // 1. If x is NaN, return "NaN".
        if self.is_nan() {
            return "NaN".into();
        }

        // 2. If x is either +0𝔽 or -0𝔽, return "0".
        if self.is_zero() {
            return "0".into();
        }

        // 3. If x < -0𝔽, return the string-concatenation of "-" and Number::toString(-x, radix).
        if self.lt(&JSNumber::from(0)) {
            return format!("-{:?}", self.clone().neg().to_string(radix)).into();
        }

        // 4. If x is +∞𝔽, return "Infinity".
        if self.is_pos_infinite() {
            return "Infinity".into();
        }

        // 5. Let n, k, and s be integers such that k ≥ 1, radix**(k - 1) ≤ s < radix**k,
        // 𝔽(s × radix**(n - k)) is x, and k is as small as possible.
        // For simplicity, we'll use a more direct approach for common cases
        // 6. If radix ≠ 10 or n is in the inclusive interval from -5 to 21, then
        // a. If n ≥ k, then
        // i. Return the string-concatenation of:
        // the code units of the k digits of the representation of s using radix radix
        // n - k occurrences of the code unit 0x0030 (DIGIT ZERO)
        // b. Else if n > 0, then
        // i. Return the string-concatenation of:
        // the code units of the most significant n digits of the representation of s using radix radix
        // the code unit 0x002E (FULL STOP)
        // the code units of the remaining k - n digits of the representation of s using radix radix
        // c. Else,
        // i. Assert: n ≤ 0.
        // ii. Return the string-concatenation of:
        // the code unit 0x0030 (DIGIT ZERO)
        // the code unit 0x002E (FULL STOP)
        // -n occurrences of the code unit 0x0030 (DIGIT ZERO)
        // the code units of the k digits of the representation of s using radix radix
        // 7. NOTE: In this case, the input will be represented using scientific E notation, such as 1.2e+3.
        // 8. Assert: radix is 10.
        // 9. If n < 0, then
        // a. Let exponentSign be the code unit 0x002D (HYPHEN-MINUS).
        // 10. Else,
        // a. Let exponentSign be the code unit 0x002B (PLUS SIGN).
        // 11. If k = 1, then
        // a. Return the string-concatenation of:
        // the code unit of the single digit of s
        // the code unit 0x0065 (LATIN SMALL LETTER E)
        // exponentSign
        // the code units of the decimal representation of abs(n - 1).
        // 12. Return the string-concatenation of:
        // the code unit of the most significant digit of the decimal representation of s
        // the code unit 0x002E (FULL STOP)
        // the code units of the remaining k - 1 digits of the decimal representation of s
        // the code unit 0x0065 (LATIN SMALL LETTER E)
        // exponentSign

        // TODO Parse the above exactly

        JSString::from(self.as_f64().to_string())
    }
}

impl TryFrom<JSString> for JSNumber {
    type Error = JSString;

    fn try_from(value: JSString) -> Result<Self, Self::Error> {
        if let Ok(number) = value.0.parse::<u32>() {
            Ok(JSNumber::UInt(number))
        } else if let Ok(number) = value.0.parse::<i32>() {
            Ok(JSNumber::Int(number))
        } else if let Ok(number) = value.0.parse::<f64>() {
            Ok(JSNumber::Float(number))
        } else {
            Err(format!("Invalid number: {}", value.0).into())
        }
    }
}

impl From<f64> for JSNumber {
    fn from(value: f64) -> Self {
        // Optimize for for i32 for memory efficiency.
        if value as i32 as f64 == value {
            return JSNumber::Int(value as i32);
        }

        JSNumber::Float(value)
    }
}

impl From<i32> for JSNumber {
    fn from(value: i32) -> Self {
        JSNumber::Int(value)
    }
}

impl From<u32> for JSNumber {
    fn from(value: u32) -> Self {
        JSNumber::UInt(value)
    }
}

/// 6.1.6.1.1 Number::unaryMinus ( x )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-unaryMinus
impl Neg for JSNumber {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // 1. If x is NaN, return NaN.
        if self.is_nan() {
            return self;
        }

        // 2. Return the negation of x; that is, compute a Number with the same magnitude but opposite sign.
        match self {
            JSNumber::Float(f) => JSNumber::Float(-f),
            JSNumber::Int(i) => JSNumber::Int(-i),
            JSNumber::UInt(u) => JSNumber::Int(-(u as i32)), // Convert unsigned to signed
        }
    }
}

/// 6.1.6.1.2 Number::bitwiseNOT ( x )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseNOT
impl Not for JSNumber {
    type Output = Self;

    fn not(self) -> Self::Output {
        JSNumber::Int(!self.as_i32())
    }
}

/// 6.1.6.1.4 Number::multiply ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-multiply
impl Mul for JSNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.is_int() && other.is_int() {
            return JSNumber::Int(self.as_i32() * other.as_i32());
        }

        (self.as_f64() * other.as_f64()).into()
    }
}
/// 6.1.6.1.5 Number::divide ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-divide
impl Div for JSNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.is_int() && other.is_int() {
            return JSNumber::Int(self.as_i32() / other.as_i32());
        }

        (self.as_f64() / other.as_f64()).into()
    }
}

/// 6.1.6.1.6 Number::remainder ( n, d )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-remainder
impl Rem for JSNumber {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        if self.is_int() && other.is_int() {
            return JSNumber::Int(self.as_i32() % other.as_i32());
        }

        (self.as_f64() % other.as_f64()).into()
    }
}

/// 6.1.6.1.7 Number::add ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-add
impl Add for JSNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.is_int() && other.is_int() {
            return JSNumber::Int(self.as_i32() + other.as_i32());
        }

        (self.as_f64() + other.as_f64()).into()
    }
}

/// 6.1.6.1.8 Number::subtract ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-subtract
impl Sub for JSNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.is_int() && other.is_int() {
            return JSNumber::Int(self.as_i32() - other.as_i32());
        }

        (self.as_f64() - other.as_f64()).into()
    }
}

/// 6.1.6.1.9 Number::leftShift ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-leftShift
impl Shl for JSNumber {
    type Output = Self;

    fn shl(self, other: Self) -> Self::Output {
        JSNumber::Int(self.as_i32() << other.as_u32())
    }
}

/// 6.1.6.1.10 Number::signedRightShift ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-signedRightShift
impl Shr for JSNumber {
    type Output = Self;

    fn shr(self, other: Self) -> Self::Output {
        JSNumber::Int(self.as_i32() >> other.as_u32())
    }
}

/// 6.1.6.1.17 Number::bitwiseAND ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseAND
impl BitAnd for JSNumber {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        JSNumber::Int(self.as_i32() & other.as_i32())
    }
}

/// 6.1.6.1.18 Number::bitwiseXOR ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseXOR
impl BitXor for JSNumber {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        JSNumber::Int(self.as_i32() ^ other.as_i32())
    }
}

/// 6.1.6.1.19 Number::bitwiseOR ( x, y )
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseOR
impl BitOr for JSNumber {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        JSNumber::Int(self.as_i32() | other.as_i32())
    }
}
