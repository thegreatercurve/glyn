use crate::{
    abstract_ops::type_conversion::{to_numeric, to_primitive, to_string, PrimitivePreferredType},
    lexer::Token,
    runtime::completion::CompletionRecord,
    value::{string::JSString, JSValue},
    JSAgent,
};

/// 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
/// https://262.ecma-international.org/15.0/#sec-applystringornumericbinaryoperator
pub(crate) fn apply_string_or_numeric_binary_operator_string_concat(
    agent: &JSAgent,
    lval: JSValue,
    rval: JSValue,
) -> CompletionRecord<JSValue> {
    // 1. If opText is +, then
    // NOTE: Implemented in the VM.

    // a. Let lprim be ? ToPrimitive(lval).
    let lprim = to_primitive(agent, lval, PrimitivePreferredType::Default)?;

    // b. Let rprim be ? ToPrimitive(rval).
    let rprim = to_primitive(agent, rval, PrimitivePreferredType::Default)?;

    // c. If lprim is a String or rprim is a String, then
    if lprim.is_string() || rprim.is_string() {
        // i. Let lstr be ? ToString(lprim).
        let lstr = to_string(agent, lprim)?;

        // ii. Let rstr be ? ToString(rprim).
        let rstr = to_string(agent, rprim)?;

        // iii. Return the string-concatenation of lstr and rstr.
        return Ok(JSValue::String(JSString::from(lstr.0 + &rstr.0)));
    }

    // d. Set lval to lprim.
    // e. Set rval to rprim.
    apply_string_or_numeric_binary_operator_numeric(agent, lprim, Token::Plus, rprim)
}

/// 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
/// https://262.ecma-international.org/15.0/#sec-applystringornumericbinaryoperator
pub(crate) fn apply_string_or_numeric_binary_operator_numeric(
    agent: &JSAgent,
    lval: JSValue,
    op_text: Token,
    rval: JSValue,
) -> CompletionRecord<JSValue> {
    // 2. NOTE: At this point, it must be a numeric operation.
    // 3. Let lnum be ? ToNumeric(lval).
    let lnum = to_numeric(agent, lval)?;

    // 4. Let rnum be ? ToNumeric(rval).
    let rnum = to_numeric(agent, rval)?;

    // 5. If Type(lnum) is not Type(rnum), throw a TypeError exception.
    if std::mem::discriminant(&lnum) != std::mem::discriminant(&rnum) {
        agent.type_error(&format!(
            "Cannot use {:?} and {:?} in a binary expression",
            lnum, rnum
        ));
    }

    // 6. If lnum is a BigInt, then
    if lnum.is_big_int() {
        // a. If opText is **, return ? BigInt::exponentiate(lnum, rnum).
        // b. If opText is /, return ? BigInt::divide(lnum, rnum).
        // c. If opText is %, return ? BigInt::remainder(lnum, rnum).
        // d. If opText is >>>, return ? BigInt::unsignedRightShift(lnum, rnum).
        todo!()
    }

    // 7. Let operation be the abstract operation associated with opText and Type(lnum) in the following table:
    // opText	Type(lnum)	operation
    match (op_text, lnum) {
        // **	Number	Number::exponentiate
        // *	Number	Number::multiply
        // *	BigInt	BigInt::multiply
        // /	Number	Number::divide
        // %	Number	Number::remainder
        // +	Number	Number::add
        (Token::Plus, JSValue::Number(lnum)) => {
            // TODo Refactor JSNumber to better handle unknown right-hand values.
            Ok(JSValue::Number(lnum + rnum.as_number().unwrap().clone()))
        }
        // +	BigInt	BigInt::add
        // -	Number	Number::subtract
        // -	BigInt	BigInt::subtract
        // <<	Number	Number::leftShift
        // <<	BigInt	BigInt::leftShift
        // >>	Number	Number::signedRightShift
        // >>	BigInt	BigInt::signedRightShift
        // >>>	Number	Number::unsignedRightShift
        // &	Number	Number::bitwiseAND
        // &	BigInt	BigInt::bitwiseAND
        // ^	Number	Number::bitwiseXOR
        // ^	BigInt	BigInt::bitwiseXOR
        // |	Number	Number::bitwiseOR
        // |	BigInt	BigInt::bitwiseOR
        _ => todo!(),
    }

    // 8. Return operation(lnum, rnum).
}
