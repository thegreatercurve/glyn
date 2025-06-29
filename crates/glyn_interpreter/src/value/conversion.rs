use crate::{JSNumber, JSValue};

impl JSValue {
    /// 7.1.2 ToBoolean ( argument )
    /// https://262.ecma-international.org/15.0/#sec-toboolean
    pub(crate) fn to_boolean(&self) -> bool {
        // 1. If argument is a Boolean, return argument.
        if let Some(value) = self.as_boolean() {
            return *value;
        }

        // 2. If argument is one of undefined, null, +0𝔽, -0𝔽, NaN, 0ℤ, or the empty String, return false.
        match self {
            JSValue::Undefined | JSValue::Null => return false,
            JSValue::Number(number) if number.is_zero() || number.is_nan() => return false,
            JSValue::String(string) if string.is_empty() => return false,
            _ => {}
        }

        // 3. NOTE: This step is replaced in section B.3.6.1.
        // 3. If argument is an Object and argument has an [[IsHTMLDDA]] internal slot, return false.
        // TODO: Implement or decide to implement annex B.

        // 4. Return true.
        true
    }
}
