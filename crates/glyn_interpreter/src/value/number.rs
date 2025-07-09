use crate::{value::string::JSString, JSValue};

/// 6.1.6.1 The Number Type
/// https://262.ecma-international.org/15.0/#sec-numeric-types-number
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JSNumber(pub f64);

impl JSNumber {
    pub(crate) const NAN: JSNumber = JSNumber(f64::NAN);

    pub(crate) const ZERO: JSNumber = JSNumber(0.0);

    pub(crate) const POS_ZERO: JSNumber = JSNumber(0.0);

    pub(crate) const NEG_ZERO: JSNumber = JSNumber(-0.0);

    pub(crate) fn is_zero(&self) -> bool {
        self.0 == 0.0 || self.is_pos_zero() || self.is_neg_zero()
    }

    fn is_pos_zero(&self) -> bool {
        self.0 == 0.0 && self.0.is_sign_positive()
    }

    fn is_neg_zero(&self) -> bool {
        self.0 == 0.0 && self.0.is_sign_negative()
    }

    pub(crate) fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    pub(crate) fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    fn is_infinite(&self) -> bool {
        self.0.is_infinite()
    }

    pub(crate) fn is_pos_infinite(&self) -> bool {
        self.0.is_infinite() && self.0 > 0.0
    }

    pub(crate) fn is_neg_infinite(&self) -> bool {
        self.0.is_infinite() && self.0 < 0.0
    }
}

impl JSNumber {
    /// 21.1.2.6 Number.MAX_SAFE_INTEGER
    /// https://262.ecma-international.org/15.0/#sec-number.max_safe_integer
    pub(crate) const MAX_SAFE_INTEGER: i64 = 2i64.pow(53) - 1;

    /// 21.1.2.7 Number.MAX_VALUE
    /// https://262.ecma-international.org/15.0/#sec-number.max_value
    pub(crate) const MAX_VALUE: f64 = f64::MAX;

    /// 21.1.2.8 Number.MIN_SAFE_INTEGER
    /// https://262.ecma-international.org/15.0/#sec-number.min_safe_integer
    pub(crate) const MIN_SAFE_INTEGER: i64 = -(2i64.pow(53) - 1);

    /// 21.1.2.9 Number.MIN_VALUE
    /// https://262.ecma-international.org/15.0/#sec-number.min_value
    pub(crate) const MIN_VALUE: f64 = f64::MIN;

    /// 6.1.6.1.1 Number::unaryMinus ( x )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-unaryMinus
    pub(crate) fn unary_minus(&self) -> Self {
        // 1. If x is NaN, return NaN.
        if self.is_nan() {
            return JSNumber::NAN;
        }

        // 2. Return the result of negating x; that is, compute a Number with the same magnitude but opposite sign.
        JSNumber(-self.0)
    }

    /// 6.1.6.1.2 Number::bitwiseNOT ( x )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseNOT
    pub(crate) fn bitwise_not(self) -> Self {
        // 1. Let oldValue be ! ToInt32(x).
        let old_value = self.0 as i32;

        // 2. Return the result of applying bitwise complement to oldValue.
        // The mathematical value of the result is exactly representable as a 32-bit two's complement bit string.
        JSNumber(!old_value as f64)
    }

    /// 6.1.6.1.3 Number::exponentiate ( base, exponent )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-exponentiate
    pub(crate) fn exponentiate(self, other: &Self) -> Self {
        // Rust and JavaScript follow IEEE 754 for floating point arithmetic, so we can use the built-in powf method.
        JSNumber(self.0.powf(other.0))
    }

    /// 6.1.6.1.4 Number::multiply ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-multiply
    pub(crate) fn multiply(self, other: Self) -> Self {
        // Rust and JavaScript follow IEEE 754 for floating point arithmetic, so we can use the built-in mult operator.
        JSNumber(self.0 * other.0)
    }

    /// 6.1.6.1.5 Number::divide ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-divide
    pub(crate) fn divide(self, other: Self) -> Self {
        // Rust and JavaScript follow IEEE 754 for floating point arithmetic, so we can use the built-in div operator.
        JSNumber(self.0 / other.0)
    }

    /// 6.1.6.1.6 Number::remainder ( n, d )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-remainder
    /// The result of a floating-point remainder operation as computed by the % operator is not the same as the “remainder” operation defined by IEEE 754-2019.
    pub(crate) fn remainder(self, other: Self) -> Self {
        // 1. If n is NaN or d is NaN, return NaN.
        if self.is_nan() || other.is_nan() {
            return JSNumber::NAN;
        }

        // 2. If n is either +∞𝔽 or -∞𝔽, return NaN.
        if self.is_infinite() {
            return JSNumber::NAN;
        }

        // 3. If d is either +∞𝔽 or -∞𝔽, return n.
        if other.is_infinite() {
            return self;
        }

        // 4. If d is either +0𝔽 or -0𝔽, return NaN.
        if other.is_zero() {
            return JSNumber::NAN;
        }

        // 5. If n is either +0𝔽 or -0𝔽, return n.
        if self.is_zero() {
            return self;
        }

        // 6. Assert: n and d are finite and non-zero.
        debug_assert!(self.is_finite() && other.is_finite() && !self.is_zero() && !other.is_zero());

        // 7. Let quotient be ℝ(n) / ℝ(d).
        let quotient = self.0 / other.0;

        // 8. Let q be truncate(quotient).
        let q = quotient.trunc();

        // 9. Let r be ℝ(n) - (ℝ(d) × q).
        let r = self.0 - (other.0 * q);

        // 10. If r = 0 and n < -0𝔽, return -0𝔽.
        if r == 0.0 && self.0 < 0.0 {
            return JSNumber::NEG_ZERO;
        }

        // 11. Return 𝔽(r).
        JSNumber(r)
    }

    /// 6.1.6.1.7 Number::add ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-add
    pub(crate) fn add(self, other: Self) -> Self {
        // Rust and JavaScript follow IEEE 754 for floating point arithmetic, so we can use the built-in add operator.
        JSNumber(self.0 + other.0)
    }

    /// 6.1.6.1.8 Number::subtract ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-subtract
    pub(crate) fn subtract(self, other: Self) -> Self {
        // Rust and JavaScript follow IEEE 754 for floating point arithmetic, so we can use the built-in sub operator.
        JSNumber(self.0 - other.0)
    }

    /// 6.1.6.1.9 Number::leftShift ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-leftShift
    pub(crate) fn left_shift(self, other: Self) -> Self {
        // 1. Let lnum be ! ToInt32(x).
        let lnum = self.0 as i32;

        // 2. Let rnum be ! ToUint32(y).
        let rnum = other.0 as u32;

        // 3. Let shiftCount be ℝ(rnum) modulo 32.
        let shift_count = rnum % 32;

        // 4. Return the result of left shifting lnum by shiftCount bits.
        // The mathematical value of the result is exactly representable as a 32-bit two's complement bit string.
        JSNumber((lnum << shift_count) as f64)
    }

    /// 6.1.6.1.10 Number::signedRightShift ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-signedRightShift
    pub(crate) fn signed_right_shift(self, other: Self) -> Self {
        // 1. Let lnum be ! ToInt32(x).
        let lnum = self.0 as i32;

        // 2. Let rnum be ! ToUint32(y).
        let rnum = other.0 as u32;

        // 3. Let shiftCount be ℝ(rnum) modulo 32.
        let shift_count = rnum % 32;

        // 4. Return the result of performing a sign-extending right shift of lnum by shiftCount bits.
        // The most significant bit is propagated. The mathematical value of the result is exactly representable as a 32-bit two's complement bit string.
        JSNumber((lnum >> shift_count) as f64)
    }

    /// 6.1.6.1.11 Number::unsignedRightShift ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-unsignedRightShift
    pub(crate) fn unsigned_right_shift(self, other: Self) -> Self {
        // 1. Let lnum be ! ToUint32(x).
        let lnum = self.0 as u32;

        // 2. Let rnum be ! ToUint32(y).
        let rnum = other.0 as u32;

        // 3. Let shiftCount be ℝ(rnum) modulo 32.
        let shift_count = rnum % 32;

        // 4. Return the result of performing a zero-filling right shift of lnum by shiftCount bits.
        // Vacated bits are filled with zero. The mathematical value of the result is exactly representable as a 32-bit unsigned bit string.
        JSNumber((lnum >> shift_count) as f64)
    }

    /// 6.1.6.1.17 Number::bitwiseAND ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseAND
    pub(crate) fn bitwise_and(self, other: Self) -> Self {
        // 6.1.6.1.16 NumberBitwiseOp ( op, x, y )
        // 1. Let lnum be ! ToInt32(x).
        let lnum = self.0 as i32;

        // 2. Let rnum be ! ToInt32(y).
        let rnum = other.0 as i32;

        // 1. Return NumberBitwiseOp(&, x, y).
        JSNumber((lnum & rnum) as f64)
    }

    /// 6.1.6.1.18 Number::bitwiseXOR ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseXOR
    pub(crate) fn bitwise_xor(self, other: Self) -> Self {
        // 6.1.6.1.16 NumberBitwiseOp ( op, x, y )
        // 1. Let lnum be ! ToInt32(x).
        let lnum = self.0 as i32;

        // 2. Let rnum be ! ToInt32(y).
        let rnum = other.0 as i32;

        // 1. Return NumberBitwiseOp(^, x, y).
        JSNumber((lnum ^ rnum) as f64)
    }

    /// 6.1.6.1.19 Number::bitwiseOR ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-bitwiseOR
    pub(crate) fn bitwise_or(self, other: Self) -> Self {
        // 6.1.6.1.16 NumberBitwiseOp ( op, x, y )
        // 1. Let lnum be ! ToInt32(x).
        let lnum = self.0 as i32;

        // 2. Let rnum be ! ToInt32(y).
        let rnum = other.0 as i32;

        // 1. Return NumberBitwiseOp(|, x, y).
        JSNumber((lnum | rnum) as f64)
    }

    /// 6.1.6.1.13 Number::equal ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-equal
    pub(crate) fn equal(&self, y: &Self) -> bool {
        // 1. If x is NaN, return false.
        // 2. If y is NaN, return false.
        if self.is_nan() || y.is_nan() {
            return false;
        };

        // 3. If x is y, return true.
        // 4. If x is +0𝔽 and y is -0𝔽, return true.
        // 5. If x is -0𝔽 and y is +0𝔽, return true.
        if self.0 == y.0
            || (self.is_pos_zero() && y.is_neg_zero())
            || (self.is_neg_zero() && y.is_pos_zero())
        {
            return true;
        }

        // 6. Return false.
        false
    }

    /// 6.1.6.1.12 Number::lessThan ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-lessthan
    pub(crate) fn less_than(&self, y: &Self) -> Option<bool> {
        // 1. If x is NaN, return undefined.
        if self.is_nan() {
            return None;
        }

        // 2. If y is NaN, return undefined.
        if y.is_nan() {
            return None;
        }

        // Rust and JavaScript follow IEEE 754 for floating point arithmetic, so we can use the built-in less than operator.
        Some(self.0 < y.0)
    }

    /// 6.1.6.1.14 Number::sameValue ( x, y )
    /// https://262.ecma-international.org/15.0/#sec-numeric-types-number-samevalue
    pub(crate) fn same_value(&self, y: &Self) -> bool {
        // 1. If x is NaN and y is NaN, return true.
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
        if self.lt(&JSNumber::ZERO) {
            return format!("-{:?}", self.clone().unary_minus().to_string(radix)).into();
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

        JSString::from(self.0.to_string())
    }
}

impl TryFrom<JSString> for JSNumber {
    type Error = JSString;

    fn try_from(value: JSString) -> Result<Self, Self::Error> {
        if let Ok(number) = value.0.parse::<f64>() {
            Ok(JSNumber(number))
        } else {
            Err(format!("Invalid number: {}", value.0).into())
        }
    }
}

impl TryFrom<&JSValue> for JSNumber {
    type Error = JSValue;

    fn try_from(value: &JSValue) -> Result<Self, Self::Error> {
        if let JSValue::Number(number) = value {
            Ok(number.clone())
        } else {
            Err(value.clone())
        }
    }
}

impl From<f64> for JSNumber {
    fn from(value: f64) -> Self {
        JSNumber(value)
    }
}

impl From<i32> for JSNumber {
    fn from(value: i32) -> Self {
        JSNumber(value as f64)
    }
}

impl From<u32> for JSNumber {
    fn from(value: u32) -> Self {
        JSNumber(value as f64)
    }
}
