use std::cmp::min;

use crate::runtime::agent::{JSAgent, WellKnownSymbol};
use crate::runtime::completion::CompletionRecord;
use crate::value::{
    number::JSNumber,
    object::{property::JSObjectPropKey, JSObjAddr},
    string::JSString,
    JSValue,
};

// 7.1 Type Conversion
// https://262.ecma-international.org/16.0/#sec-type-conversion

pub(crate) enum PreferredPrimType {
    Default,
    String,
    Number,
}

/// 7.1.1 ToPrimitive ( input [ , preferredType ] )
/// https://262.ecma-international.org/16.0/#sec-toprimitive
pub(crate) fn to_primitive(
    agent: &JSAgent,
    input: JSValue,
    preferred_type: PreferredPrimType,
) -> CompletionRecord<JSValue> {
    let mut preferred_type = preferred_type;

    // 1. If input is an Object, then
    if let Some(obj_addr) = input.as_object() {
        // a. Let exoticToPrim be ? GetMethod(input, @@toPrimitive).
        let exotic_to_prim = agent.well_known_symbol(obj_addr, WellKnownSymbol::ToPrimitive);

        // b. If exoticToPrim is not undefined, then
        if let Some(exotic_to_prim) = exotic_to_prim {
            preferred_type = match preferred_type {
                // i. If preferredType is not present, then
                // 1. Let hint be "default".
                PreferredPrimType::Default => PreferredPrimType::Default,
                // ii. Else if preferredType is string, then
                // 1. Let hint be "string".
                PreferredPrimType::String => PreferredPrimType::String,
                // iii. Else,
                // 1. Assert: preferredType is number.
                // 2. Let hint be "number".
                PreferredPrimType::Number => PreferredPrimType::Number,
            };

            todo!();

            // iv. Let result be ? Call(exoticToPrim, input, « hint »).
            // v. If result is not an Object, return result.
            // vi. Throw a TypeError exception
        }

        // c. If preferredType is not present, let preferredType be number.
        if matches!(preferred_type, PreferredPrimType::Default) {
            preferred_type = PreferredPrimType::Number;
        }

        // d. Return ? OrdinaryToPrimitive(input, preferredType).
        todo!()
    }

    // 2. Return input.
    Ok(input)
}

/// 7.1.2 ToBoolean ( argument )
/// https://262.ecma-international.org/16.0/#sec-toboolean
pub(crate) fn to_boolean(agent: &JSAgent, arg: JSValue) -> bool {
    // 1. If argument is a Boolean, return argument.
    if let Some(value) = arg.as_boolean() {
        return *value;
    }

    // 2. If argument is one of undefined, null, +0𝔽, -0𝔽, NaN, 0ℤ, or the empty String, return false.
    match arg {
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

/// 7.1.3 ToNumeric ( value )
/// https://262.ecma-international.org/16.0/#sec-tonumeric
pub(crate) fn to_numeric(agent: &JSAgent, value: JSValue) -> CompletionRecord<JSValue> {
    // 1. Let primValue be ? ToPrimitive(value, number).
    let prim_value = to_primitive(agent, value, PreferredPrimType::Number)?;

    // 2. If primValue is a BigInt, return primValue.
    if prim_value.is_big_int() {
        return Ok(prim_value);
    }

    // 3. Return ? ToNumber(primValue).
    Ok(JSValue::Number(to_number(agent, prim_value)?))
}

/// 7.1.4 ToNumber ( argument )
/// https://262.ecma-international.org/16.0/#sec-tonumber
pub(crate) fn to_number(agent: &JSAgent, arg: JSValue) -> CompletionRecord<JSNumber> {
    match arg {
        // 1. If argument is a Number, return argument.
        JSValue::Number(number) => return Ok(number.clone()),
        // 2. If argument is either a Symbol or a BigInt, throw a TypeError exception.
        JSValue::Symbol(_) => agent.type_error("Cannot convert Symbol to JSNumber"),
        JSValue::BigInt(_) => agent.type_error("Cannot convert BigInt to JSNumber"),
        // 3. If argument is undefined, return NaN.
        JSValue::Undefined => return Ok(JSNumber::NAN),
        // 4. If argument is either null or false, return +0𝔽.
        JSValue::Null | JSValue::Bool(false) => return Ok(JSNumber::ZERO),
        // 5. If argument is true, return +1𝔽
        JSValue::Bool(true) => return Ok(JSNumber(1.0)),
        // 6. If argument is a String, return StringToNumber(argument).
        JSValue::String(ref string) => return Ok(string_to_number(agent, string)),
        _ => {}
    };

    // 7. Assert: argument is an Object.
    debug_assert!(arg.is_object());

    // 8. Let primValue be ? ToPrimitive(argument, number).
    let prim_value = to_primitive(agent, arg, PreferredPrimType::Number)?;

    // 9. Assert: primValue is not an Object.
    debug_assert!(!prim_value.is_object());

    // 10. Return ? ToNumber(primValue).
    to_number(agent, prim_value)
}

/// 7.1.4.1.1 StringToNumber ( str )
/// https://262.ecma-international.org/16.0/#sec-stringtonumber
pub(crate) fn string_to_number(_agent: &JSAgent, str: &JSString) -> JSNumber {
    // 1. Let text be StringToCodePoints(str).
    // 2. Let literal be ParseText(text, StringNumericLiteral).
    // TODO Implement the below exactly.
    let literal = str.0.parse::<f64>();

    // 3. If literal is a List of errors, return NaN.
    let Ok(literal) = literal else {
        return JSNumber::NAN;
    };

    // 4. Return StringNumericValue of literal.
    JSNumber::from(literal)
}
/// https://262.ecma-international.org/16.0/#sec-tointegerorinfinity
pub(crate) fn to_integer_or_infinity(
    agent: &JSAgent,
    argument: JSValue,
) -> CompletionRecord<JSNumber> {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, argument)?;

    // 2. If number is one of NaN, +0𝔽, or -0𝔽, return 0.
    if number.is_nan() || number.is_zero() {
        return Ok(JSNumber::ZERO);
    }

    // 3. If number is +∞𝔽, return +∞.
    if number.is_pos_infinite() {
        return Ok(JSNumber(f64::INFINITY));
    }

    // 4. If number is -∞𝔽, return -∞.
    if number.is_neg_infinite() {
        return Ok(JSNumber(f64::NEG_INFINITY));
    }

    // 5. Return truncate(ℝ(number)).
    Ok(JSNumber(number.0.trunc()))
}

/// 7.1.6 ToInt32 ( argument )
/// https://262.ecma-international.org/16.0/#sec-toint32
pub(crate) fn to_int32(agent: &JSAgent, argument: JSValue) -> CompletionRecord<JSNumber> {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, argument)?;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    // 3. Let int be truncate(ℝ(number)).
    // 4. Let int32bit be int modulo 2^32.
    // 5. If int32bit ≥ 2^31, return 𝔽(int32bit - 2^32); otherwise return 𝔽(int32bit).
    Ok(JSNumber(number.0 as i32 as f64))
}

/// 7.1.7 ToUint32 ( argument )
/// https://262.ecma-international.org/16.0/#sec-touint32
pub(crate) fn to_uint32(agent: &JSAgent, argument: JSValue) -> CompletionRecord<JSNumber> {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, argument)?;

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    // 3. Let int be truncate(ℝ(number)).
    // 4. Let int32bit be int modulo 2^32.
    // 5. Return 𝔽(int32bit).
    Ok(JSNumber(number.0 as u32 as f64))
}

/// 7.1.17 ToString ( argument )
/// https://262.ecma-international.org/16.0/#sec-tostring
pub(crate) fn to_string(agent: &JSAgent, argument: JSValue) -> CompletionRecord<JSString> {
    // 1. If argument is a String, return argument.
    if let Some(string) = argument.as_string() {
        return Ok(string.clone());
    }

    // 2. If argument is a Symbol, throw a TypeError exception.
    if argument.is_symbol() {
        agent.type_error("Cannot convert Symbol to string");
    }

    // 3. If argument is undefined, return "undefined".
    if argument == JSValue::Undefined {
        return Ok(JSString::from("undefined"));
    }

    // 4. If argument is null, return "null".
    if argument == JSValue::Null {
        return Ok(JSString::from("null"));
    }

    // 5. If argument is true, return "true".
    if argument == JSValue::Bool(true) {
        return Ok(JSString::from("true"));
    }

    // 6. If argument is false, return "false".
    if argument == JSValue::Bool(false) {
        return Ok(JSString::from("false"));
    }

    // 7. If argument is a Number, return Number::toString(argument, 10).
    if let JSValue::Number(number) = argument {
        return Ok(number.to_string(10));
    }

    // 8. If argument is a BigInt, return BigInt::toString(argument, 10).
    if let JSValue::BigInt(big_int) = argument {
        return Ok(big_int.to_string(10));
    }

    // 9. Assert: argument is an Object.
    debug_assert!(argument.is_object());

    // 10. Let primValue be ? ToPrimitive(argument, string).
    let prim_value = to_primitive(agent, argument, PreferredPrimType::String)?;

    // 11. Assert: primValue is not an Object.
    debug_assert!(!prim_value.is_object());

    // 12. Return ? ToString(primValue).
    to_string(agent, prim_value)
}

/// 7.1.18 ToObject ( argument )
/// https://262.ecma-international.org/16.0/#sec-toobject
pub(crate) fn to_object(agent: &JSAgent, arg: &JSValue) -> JSObjAddr {
    match arg {
        JSValue::Undefined => {
            // Throw a TypeError exception.
            agent.type_error("Cannot convert undefined to object");
        }
        JSValue::Null => {
            // Throw a TypeError exception.
            agent.type_error("Cannot convert null to object");
        }
        // Return a new Boolean object whose [[BooleanData]] internal slot is set to argument.
        JSValue::Bool(_value) => todo!(),
        // Return a new Number object whose [[NumberData]] internal slot is set to argument.
        JSValue::Number(_value) => todo!(),
        // Return a new String object whose [[StringData]] internal slot is set to argument.
        JSValue::String(_value) => todo!(),
        // Return a new Symbol object whose [[SymbolData]] internal slot is set to argument.
        JSValue::Symbol(_) => todo!(),
        // Return a new BigInt object whose [[BigIntData]] internal slot is set to argument.
        JSValue::BigInt(_value) => todo!(),
        // If argument is an Object, return argument.
        JSValue::Object(addr) => *addr,
    }
}

/// 7.1.19 ToPropertyKey ( argument )
/// https://262.ecma-international.org/16.0/#sec-topropertykey
pub(crate) fn to_property_key(
    agent: &JSAgent,
    argument: JSValue,
) -> CompletionRecord<JSObjectPropKey> {
    // 1. Let key be ? ToPrimitive(argument, string).
    let key = to_primitive(agent, argument, PreferredPrimType::String)?;

    // 2. If key is a Symbol, then
    if let Some(symbol) = key.as_symbol() {
        // a. Return key.
        return Ok(JSObjectPropKey::Symbol(symbol.clone()));
    }

    // 3. Return ! ToString(key).
    Ok(JSObjectPropKey::String(to_string(agent, key)?))
}

/// 7.1.20 ToLength ( argument )
/// https://262.ecma-international.org/16.0/#sec-tolength
pub(crate) fn to_length(agent: &JSAgent, argument: JSValue) -> CompletionRecord<JSNumber> {
    // 1. Let len be ? ToIntegerOrInfinity(argument).
    let len = to_integer_or_infinity(agent, argument)?;

    // 2. If len ≤ 0, return +0𝔽.
    if len.lt(&JSNumber::ZERO) {
        return Ok(JSNumber::ZERO);
    }

    // 3. Return 𝔽(min(len, 2^53 - 1)).
    Ok(JSNumber(
        min(len.0 as i64, JSNumber::MAX_SAFE_INTEGER) as f64
    ))
}

/// 7.1.21 CanonicalNumericIndexString ( argument )
/// https://262.ecma-international.org/16.0/#sec-canonicalnumericindexstring
pub(crate) fn canonical_numeric_index_string(
    agent: &JSAgent,
    argument: &JSString,
) -> Option<JSNumber> {
    // 1. If argument is "-0", return -0𝔽.
    if argument.0 == "-0" {
        return Some(JSNumber::NEG_ZERO);
    }

    // 2. Let n be ! ToNumber(argument).
    let Ok(n) = to_number(agent, JSValue::from(argument.clone())) else {
        return None;
    };

    // 3. If ! ToString(n) is argument, return n.
    let Ok(string) = to_string(agent, JSValue::from(n.clone())) else {
        return None;
    };

    if string == *argument {
        return Some(n);
    }

    // 4. Return undefined.
    None
}

/// 7.1.22 ToIndex ( value )
/// https://262.ecma-international.org/16.0/#sec-toindex
pub(crate) fn to_index(agent: &JSAgent, value: JSValue) -> CompletionRecord<JSNumber> {
    // 1. Let integer be ? ToIntegerOrInfinity(value).
    let integer = to_integer_or_infinity(agent, value)?;

    // 2. If integer is not in the inclusive interval from 0 to 2^53 - 1, throw a RangeError exception.
    if integer < JSNumber::ZERO || integer > JSNumber::from(JSNumber::MAX_SAFE_INTEGER as f64) {
        agent.range_error("Index must be in the range 0 - 2^53-1");
    }

    // 3. Return integer.
    Ok(integer)
}
