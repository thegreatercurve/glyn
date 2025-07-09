use crate::abstract_ops::type_conversion::{to_numeric, to_primitive, PrimitivePreferredType};
use crate::runtime::agent::JSAgent;
use crate::runtime::completion::CompletionRecord;
use crate::value::{number::JSNumber, object::JSObjAddr, JSValue};

// 7.2 Testing and Comparison Operations
// https://262.ecma-international.org/15.0/#sec-testing-and-comparison-operations

/// 7.2.1 RequireObjectCoercible ( argument )
/// https://262.ecma-international.org/15.0/#sec-requireobjectcoercible
pub(crate) fn require_object_coercible(agent: &JSAgent, arg: JSValue) -> CompletionRecord<JSValue> {
    //  It throws an error if argument is a value that cannot be converted to an Object using ToObject (e.g. null or undefined).
    if arg.is_null() || arg.is_undefined() {
        agent.type_error("Cannot convert null or undefined to object");
    }

    Ok(arg)
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
        JSNumber(value) => &value.trunc() == value,
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
    arg.is_string() || arg.is_symbol()
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
        (JSValue::Bool(x), JSValue::Bool(y)) => x == y,

        // 6. NOTE: All other ECMAScript language values are compared by identity.
        (JSValue::Number(_x), JSValue::Number(_y)) => unreachable!(),
        (JSValue::Object(x), JSValue::Object(y)) => x == y,
        (JSValue::Symbol(x), JSValue::Symbol(y)) => x == y,

        // 7. If x is y, return true; otherwise, return false.
        _ => false,
    }
}

/// 7.2.13 IsLessThan ( x, y, LeftFirst )
/// https://262.ecma-international.org/15.0/#sec-islessthan
pub(crate) fn is_less_than(
    agent: &JSAgent,
    x: JSValue,
    y: JSValue,
    left_first: bool,
) -> CompletionRecord<Option<bool>> {
    let px;
    let py;

    // 1. If LeftFirst is true, then
    if left_first {
        // a. Let px be ? ToPrimitive(x, number).
        px = to_primitive(agent, x, PrimitivePreferredType::Number)?;

        // b. Let py be ? ToPrimitive(y, number).
        py = to_primitive(agent, y, PrimitivePreferredType::Number)?;
    }
    // 2. Else,
    else {
        // a. NOTE: The order of evaluation needs to be reversed to preserve left to right evaluation.
        // b. Let py be ? ToPrimitive(y, number).
        py = to_primitive(agent, y, PrimitivePreferredType::Number)?;

        // c. Let px be ? ToPrimitive(x, number).
        px = to_primitive(agent, x, PrimitivePreferredType::Number)?;
    }

    // 3. If px is a String and py is a String, then
    if let (Some(px_str), Some(py_str)) = (px.as_string(), py.as_string()) {
        // a. Let lx be the length of px.
        let lx = px_str.utf16_len();

        // b. Let ly be the length of py.
        let ly = py_str.utf16_len();

        let px_chars = px_str.0.chars().collect::<Vec<_>>();
        let py_chars = py_str.0.chars().collect::<Vec<_>>();

        // c. For each integer i such that 0 ≤ i < min(lx, ly), in ascending order, do
        for i in 0..lx.min(ly) {
            // i. Let cx be the numeric value of the code unit at index i within px.
            let cx = px_chars[i] as u32;

            // ii. Let cy be the numeric value of the code unit at index i within py.
            let cy = py_chars[i] as u32;

            // iii. If cx < cy, return true.
            if cx < cy {
                return Ok(Some(true));
            }

            // iv. If cx > cy, return false.
            if cx > cy {
                return Ok(Some(false));
            }
        }

        // d. If lx < ly, return true. Otherwise, return false.
        Ok(Some(lx < ly))
    }
    // 4. Else,
    else {
        // a. If px is a BigInt and py is a String, then
        if let (Some(px_bigint), Some(py_str)) = (px.as_big_int(), py.as_string()) {
            // i. Let ny be StringToBigInt(py).
            // ii. If ny is undefined, return undefined.
            // iii. Return BigInt::lessThan(px, ny).
            todo!()
        }

        // b. If px is a String and py is a BigInt, then
        if let (Some(px_str), Some(py_bigint)) = (px.as_string(), py.as_big_int()) {
            // i. Let nx be StringToBigInt(px).
            // ii. If nx is undefined, return undefined.
            // iii. Return BigInt::lessThan(nx, py).
            todo!()
        }

        // c. NOTE: Because px and py are primitive values, evaluation order is not important.
        // d. Let nx be ? ToNumeric(px).
        let nx = to_numeric(agent, px)?;

        // e. Let ny be ? ToNumeric(py).
        let ny = to_numeric(agent, py)?;

        // f. If Type(nx) is Type(ny), then
        if std::mem::discriminant(&nx) == std::mem::discriminant(&ny) {
            // i. If nx is a Number, then
            if let (Some(nx_num), Some(ny_num)) = (nx.as_number(), ny.as_number()) {
                // 1. Return Number::lessThan(nx, ny).
                return Ok(nx_num.less_than(ny_num));
            }
            // ii. Else,
            else {
                // 1. Assert: nx is a BigInt.
                // 2. Return BigInt::lessThan(nx, ny).
                todo!()
            }
        }

        // g. Assert: nx is a BigInt and ny is a Number, or nx is a Number and ny is a BigInt.
        debug_assert!((nx.is_big_int() && ny.is_number()) || (nx.is_number() && ny.is_big_int()));

        // h. If nx or ny is NaN, return undefined.
        if nx.is_nan() || ny.is_nan() {
            return Ok(None);
        }

        // i. If nx is -∞𝔽 or ny is +∞𝔽, return true.
        if nx.is_neg_infinite() || ny.is_pos_infinite() {
            return Ok(Some(true));
        }

        // j. If nx is +∞𝔽 or ny is -∞𝔽, return false.
        if nx.is_pos_infinite() || ny.is_neg_infinite() {
            return Ok(Some(false));
        }

        // k. If ℝ(nx) < ℝ(ny), return true; otherwise return false.
        Ok(Some(
            nx.as_number().unwrap_or_else(|| unreachable!()).0
                < ny.as_number().unwrap_or_else(|| unreachable!()).0,
        ))
    }
}

/// 7.2.15 IsStrictlyEqual ( x, y )
/// https://262.ecma-international.org/15.0/#sec-isstrictlyequal
pub(crate) fn is_strictly_equal(x: &JSValue, y: &JSValue) -> bool {
    // 1. If Type(x) is different from Type(y), return false.
    if std::mem::discriminant(x) != std::mem::discriminant(y) {
        return false;
    }

    // 2. If x is a Number, then
    if let (Some(x_num), Some(y_num)) = (x.as_number(), y.as_number()) {
        // a. Return Number::equal(x, y).
        return x_num.equal(y_num);
    }

    // 3. Return SameValueNonNumber(x, y).
    same_value_non_number(x, y)
}
