use crate::{
    runtime::{
        agent::JSAgent,
        completion::{CompletionRecord, NormalCompletion},
    },
    value::{number::JSNumber, object::JSObjAddr, JSValue},
};

// 7.2 Testing and Comparison Operations
// https://262.ecma-international.org/15.0/#sec-testing-and-comparison-operations

/// 7.2.1 RequireObjectCoercible ( argument )
/// https://262.ecma-international.org/15.0/#sec-requireobjectcoercible
pub(crate) fn require_object_coercible(agent: &JSAgent, arg: JSValue) -> CompletionRecord {
    //  It throws an error if argument is a value that cannot be converted to an Object using ToObject (e.g. null or undefined).
    if matches!(arg, JSValue::Null | JSValue::Undefined) {
        agent.type_error("Cannot convert null or undefined to object");
    }

    Ok(NormalCompletion::Value(arg))
}

/// 7.2.3 IsCallable ( argument )
/// https://262.ecma-international.org/15.0/#sec-iscallable
pub(crate) fn is_callable(agent: &JSAgent, arg: &JSValue) -> bool {
    // If argument is not an Object, return false.
    let Some(obj_addr) = arg.as_object() else {
        return false;
    };

    // 2. If argument has a [[Call]] internal method, return true.
    if agent.object(obj_addr).methods.call.is_some() {
        return true;
    }

    // 3. Return false.
    false
}

/// 7.2.4 IsConstructor ( argument )
/// https://262.ecma-international.org/15.0/#sec-isconstructor
pub(crate) fn is_constructor(agent: &JSAgent, arg: JSValue) -> bool {
    // If argument is not an Object, return false.
    let Some(obj_addr) = arg.as_object() else {
        return false;
    };

    // 2. If argument has a [[Construct]] internal method, return true.
    if agent.object(obj_addr).methods.construct.is_some() {
        return true;
    }

    // 3. Return false.
    false
}

///  7.2.5 IsExtensible ( O )
/// https://262.ecma-international.org/15.0/#sec-isextensible-o
pub(crate) fn is_extensible(agent: &JSAgent, obj_addr: JSObjAddr) -> bool {
    // 1. Return O.[[Extensible]].
    agent.object(obj_addr).extensible()
}

/// 7.2.6 IsIntegralNumber ( argument )
/// https://262.ecma-international.org/15.0/#sec-isintegralnumber
pub(crate) fn is_integral_number(_agent: &JSAgent, arg: JSValue) -> bool {
    // 1. If argument is not a Number, return false.
    let Some(number) = arg.as_number() else {
        return false;
    };

    // 2. If argument is not finite, return false.
    if !number.is_finite() {
        return false;
    }

    match number {
        // 3. If truncate(ℝ(argument)) ≠ ℝ(argument), return false.
        JSNumber::Float(value) => &value.trunc() == value,
        // 4. Return true.
        _ => true,
    }
}

/// 7.2.7 IsPropertyKey ( argument )
/// https://262.ecma-international.org/15.0/#sec-ispropertykey
pub(crate) fn is_property_key(_agent: &JSAgent, arg: JSValue) -> bool {
    // 1. If Type(argument) is String, return true.
    // 2. If Type(argument) is Symbol, return false.
    // 3. Return false.
    matches!(arg, JSValue::String(_) | JSValue::Symbol)
}

/// 7.2.10 SameValue ( x, y )
/// https://262.ecma-international.org/15.0/#sec-samevalue
pub(crate) fn same_value(x: &JSValue, y: &JSValue) -> bool {
    // 1. If Type(x) is not Type(y), return false.
    if std::mem::discriminant(x) != std::mem::discriminant(y) {
        return false;
    }

    // 2. If x is a Number, then
    if let JSValue::Number(x) = x {
        // a. Return Number::sameValue(x, y).
        return x.same_value(y.as_number().unwrap_or_else(|| unreachable!()));
    }

    // 3. Return SameValueNonNumber(x, y).
    same_value_non_number(x, y)
}

/// 7.2.12 SameValueNonNumber ( x, y )
/// https://262.ecma-international.org/15.0/#sec-samevaluenonnumber
fn same_value_non_number(x: &JSValue, y: &JSValue) -> bool {
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
        (JSValue::Object(x), JSValue::Object(y)) => x == y,
        (JSValue::Symbol, JSValue::Symbol) => todo!(),

        // 7. If x is y, return true; otherwise, return false.
        _ => false,
    }
}
