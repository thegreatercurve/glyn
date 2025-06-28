use crate::{JSAgent, JSValue};

/// 7.2.10 SameValue ( x, y )
/// https://262.ecma-international.org/15.0/#sec-samevalue
pub(crate) fn same_value(agent: &JSAgent, x: &JSValue, y: &JSValue) -> bool {
    // 1. If Type(x) is not Type(y), return false.
    if std::mem::discriminant(x) != std::mem::discriminant(y) {
        return false;
    }

    // 2. If x is a Number, then
    if let JSValue::Number(x) = x {
        // a. Return Number::sameValue(x, y).
        return x.same_value(y.as_number());
    }

    // 3. Return SameValueNonNumber(x, y).
    same_value_non_number(agent, x, y)
}

/// 7.2.12 SameValueNonNumber ( x, y )
/// https://262.ecma-international.org/15.0/#sec-samevaluenonnumber
fn same_value_non_number(agent: &JSAgent, x: &JSValue, y: &JSValue) -> bool {
    // 1. Assert: Type(x) is Type(y).
    match (x, y) {
        // 2. If x is either null or undefined, return true.
        (JSValue::Null, JSValue::Null) => true,
        (JSValue::Undefined, JSValue::Undefined) => true,

        // 3. If x is a BigInt, then
        // a. Return BigInt::equal(x, y).
        (JSValue::BigInt(_x), JSValue::BigInt(_y)) => todo!(),

        // 4. If x is a String, then
        // a. If x and y have the same length and the same code units in the same positions, return true; otherwise, return false.
        (JSValue::String(x), JSValue::String(y)) => x == y,

        // 5. If x is a Boolean, then
        (JSValue::Boolean(x), JSValue::Boolean(y)) => x == y,

        // 6. NOTE: All other ECMAScript language values are compared by identity.
        (JSValue::Number(_x), JSValue::Number(_y)) => unreachable!(),
        (JSValue::Object(x), JSValue::Object(y)) => {
            agent.deref_object_ptr(*x) == agent.deref_object_ptr(*y)
        }
        (JSValue::Symbol, JSValue::Symbol) => todo!(),

        // 7. If x is y, return true; otherwise, return false.
        _ => false,
    }
}
