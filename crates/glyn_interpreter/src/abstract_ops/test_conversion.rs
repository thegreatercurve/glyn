use crate::{
    runtime::agent::{JSAgent, WellKnownSymbol},
    value::{number::JSNumber, string::JSString, JSValue},
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
