use crate::{
    runtime::agent::{JSAgent, WellKnownSymbol},
    value::{number::JSNumber, object::JSObjAddr, string::JSString, JSValue},
};

enum PrimitivePreferredType {
    Default,
    String,
    Number,
}

// 7.1 Type Conversion
// https://262.ecma-international.org/15.0/#sec-type-conversion

/// 7.1.1 ToPrimitive ( input [ , preferredType ] )
/// https://262.ecma-international.org/15.0/#sec-toprimitive
fn to_primitive(
    agent: &JSAgent,
    input: JSValue,
    mut preferred_type: PrimitivePreferredType,
) -> JSValue {
    // 1. If input is an Object, then
    if let Some(obj_addr) = input.as_object() {
        // a. Let exoticToPrim be ? GetMethod(input, @@toPrimitive).
        let exotic_to_prim = agent.well_known_symbol(obj_addr, WellKnownSymbol::ToPrimitive);

        // b. If exoticToPrim is not undefined, then
        if let Some(exotic_to_prim) = exotic_to_prim {
            preferred_type = match preferred_type {
                // i. If preferredType is not present, then
                // 1. Let hint be "default".
                PrimitivePreferredType::Default => PrimitivePreferredType::Default,
                // ii. Else if preferredType is string, then
                // 1. Let hint be "string".
                PrimitivePreferredType::String => PrimitivePreferredType::String,
                // iii. Else,
                // 1. Assert: preferredType is number.
                // 2. Let hint be "number".
                PrimitivePreferredType::Number => PrimitivePreferredType::Number,
            };

            todo!();

            // iv. Let result be ? Call(exoticToPrim, input, « hint »).
            // v. If result is not an Object, return result.
            // vi. Throw a TypeError exception
        }

        // c. If preferredType is not present, let preferredType be number.
        if matches!(preferred_type, PrimitivePreferredType::Default) {
            preferred_type = PrimitivePreferredType::Number;
        }

        // d. Return ? OrdinaryToPrimitive(input, preferredType).
        todo!()
    }

    // 2. Return input.
    input
}

/// 7.1.2 ToBoolean ( argument )
/// https://262.ecma-international.org/15.0/#sec-toboolean
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
/// https://262.ecma-international.org/15.0/#sec-tonumeric
pub(crate) fn to_numeric(agent: &JSAgent, value: JSValue) -> JSValue {
    // 1. Let primValue be ? ToPrimitive(value, number).
    let prim_value = to_primitive(agent, value, PrimitivePreferredType::Number);

    // 2. If primValue is a BigInt, return primValue.
    if prim_value.is_big_int() {
        return prim_value;
    }

    // 3. Return ? ToNumber(primValue).
    JSValue::Number(to_number(agent, prim_value))
}

/// 7.1.4 ToNumber ( argument )
/// https://262.ecma-international.org/15.0/#sec-tonumber
pub(crate) fn to_number(agent: &JSAgent, arg: JSValue) -> JSNumber {
    match arg {
        // 1. If argument is a Number, return argument.
        JSValue::Number(number) => return number.clone(),
        // 2. If argument is either a Symbol or a BigInt, throw a TypeError exception.
        JSValue::Symbol(_) => agent.type_error("Cannot convert Symbol to JSNumber"),
        JSValue::BigInt(_) => agent.type_error("Cannot convert BigInt to JSNumber"),
        // 3. If argument is undefined, return NaN.
        JSValue::Undefined => return JSNumber::nan(),
        // 4. If argument is either null or false, return +0𝔽.
        JSValue::Null | JSValue::Bool(false) => return JSNumber::from(0),
        // 5. If argument is true, return +1𝔽
        JSValue::Bool(true) => return JSNumber::from(1),
        // 6. If argument is a String, return StringToNumber(argument).
        JSValue::String(ref string) => return string_to_number(agent, string),
        _ => {}
    };

    // 7. Assert: argument is an Object.
    debug_assert!(arg.is_object(),);

    // 8. Let primValue be ? ToPrimitive(argument, number).
    let prim_value = to_primitive(agent, arg, PrimitivePreferredType::Number);

    // 9. Assert: primValue is not an Object.
    debug_assert!(!prim_value.is_object());

    // 10. Return ? ToNumber(primValue).
    to_number(agent, prim_value)
}

/// 7.1.4.1.1 StringToNumber ( str )
/// https://262.ecma-international.org/15.0/#sec-stringtonumber
pub(crate) fn string_to_number(_agent: &JSAgent, str: &JSString) -> JSNumber {
    // 1. Let text be StringToCodePoints(str).
    // 2. Let literal be ParseText(text, StringNumericLiteral).
    // TODO Implement the below exactly.
    let literal = str.0.parse::<f64>();

    // 3. If literal is a List of errors, return NaN.
    let Ok(literal) = literal else {
        return JSNumber::nan();
    };

    // 4. Return StringNumericValue of literal.
    JSNumber::from(literal)
}
/// https://262.ecma-international.org/15.0/#sec-tointegerorinfinity
pub(crate) fn to_integer_or_infinity(agent: &JSAgent, argument: JSValue) -> JSNumber {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, argument);

    // 2. If number is one of NaN, +0𝔽, or -0𝔽, return 0.
    if number.is_nan() || number.is_zero() {
        return JSNumber::from(0);
    }

    // 3. If number is +∞𝔽, return +∞.
    if number.is_pos_infinite() {
        return JSNumber::Float(f64::INFINITY);
    }

    // 4. If number is -∞𝔽, return -∞.
    if number.is_neg_infinite() {
        return JSNumber::Float(f64::NEG_INFINITY);
    }

    // 5. Return truncate(ℝ(number)).
    number.truncate()
}

/// 7.1.6 ToInt32 ( argument )
/// https://262.ecma-international.org/15.0/#sec-toint32
pub(crate) fn to_int32(agent: &JSAgent, argument: JSValue) -> JSNumber {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, argument);

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !number.is_finite() || number.is_zero() {
        return JSNumber::Int(0);
    }

    // 3. Let int be truncate(ℝ(number)).
    let int = number.truncate();

    // 4. Let int32bit be int modulo 2^32.
    let int32bit = int % JSNumber::Int(2i32.pow(32));

    // 5. If int32bit ≥ 2^31, return 𝔽(int32bit - 2^32); otherwise return 𝔽(int32bit).
    if int32bit >= JSNumber::Int(2i32.pow(31)) {
        int32bit - JSNumber::Int(2i32.pow(32))
    } else {
        int32bit
    }
}

/// 7.1.7 ToUint32 ( argument )
/// https://262.ecma-international.org/15.0/#sec-touint32
pub(crate) fn to_uint32(agent: &JSAgent, argument: JSValue) -> JSNumber {
    // 1. Let number be ? ToNumber(argument).
    let number = to_number(agent, argument);

    // 2. If number is not finite or number is either +0𝔽 or -0𝔽, return +0𝔽.
    if !number.is_finite() || number.is_zero() {
        return JSNumber::UInt(0);
    }

    // 3. Let int be truncate(ℝ(number)).
    let int = number.truncate();

    // 4. Let int32bit be int modulo 2^32.
    // 5. Return 𝔽(int32bit).
    int % JSNumber::UInt(2u32.pow(32))
}

/// 7.1.17 ToString ( argument )
/// https://262.ecma-international.org/15.0/#sec-tostring
pub(crate) fn to_string(agent: &JSAgent, argument: JSValue) -> JSString {
    // 1. If argument is a String, return argument.
    if argument.is_string() {
        return argument
            .as_string()
            .unwrap_or_else(|| unreachable!())
            .clone();
    }

    // 2. If argument is a Symbol, throw a TypeError exception.
    if argument.is_symbol() {
        agent.type_error("Cannot convert Symbol to string");
    }

    // 3. If argument is undefined, return "undefined".
    if argument == JSValue::Undefined {
        return JSString::from("undefined");
    }

    // 4. If argument is null, return "null".
    if argument == JSValue::Null {
        return JSString::from("null");
    }

    // 5. If argument is true, return "true".
    if argument == JSValue::Bool(true) {
        return JSString::from("true");
    }

    // 6. If argument is false, return "false".
    if argument == JSValue::Bool(false) {
        return JSString::from("false");
    }

    // 7. If argument is a Number, return Number::toString(argument, 10).
    if let JSValue::Number(number) = argument {
        return number.to_string(10);
    }

    // 8. If argument is a BigInt, return BigInt::toString(argument, 10).
    if let JSValue::BigInt(big_int) = argument {
        return big_int.to_string(10);
    }

    // 9. Assert: argument is an Object.
    debug_assert!(argument.is_object());

    // 10. Let primValue be ? ToPrimitive(argument, string).
    let prim_value = to_primitive(agent, argument, PrimitivePreferredType::String);

    // 11. Assert: primValue is not an Object.
    debug_assert!(!prim_value.is_object());

    // 12. Return ? ToString(primValue).
    to_string(agent, prim_value)
}

/// 7.1.18 ToObject ( argument )
/// https://262.ecma-international.org/15.0/#sec-toobject
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
