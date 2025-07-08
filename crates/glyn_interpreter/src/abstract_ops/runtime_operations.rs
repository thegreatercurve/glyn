use crate::{
    abstract_ops::type_conversion::{to_numeric, to_primitive, to_string, PrimitivePreferredType},
    lexer::Token,
    runtime::completion::CompletionRecord,
    value::{number::JSNumber, string::JSString, JSValue},
    JSAgent,
};

/// 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
/// https://262.ecma-international.org/15.0/#sec-applystringornumericbinaryoperator
pub(crate) fn apply_string_or_numeric_binary_operator(
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
    apply_numeric_binary_operator(agent, lprim, Token::Plus, rprim)
}

/// 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
/// https://262.ecma-international.org/15.0/#sec-applystringornumericbinaryoperator
pub(crate) fn apply_numeric_binary_operator(
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

        // 7. Let operation be the abstract operation associated with opText and Type(lnum) in the following table:
        // opText	Type(lnum)	operation
        // let op_result = match (op_text, lnum, rnum) {
        //     // *	BigInt	BigInt::multiply
        //     (Token::Multiply, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // +	BigInt	BigInt::add
        //     (Token::Plus, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // -	BigInt	BigInt::subtract
        //     (Token::Minus, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // <<	BigInt	BigInt::leftShift
        //     (Token::LeftShift, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // >>	BigInt	BigInt::signedRightShift
        //     (Token::RightShift, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // >>>	Number	Number::unsignedRightShift
        //     (Token::UnsignedRightShift, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // &	BigInt	BigInt::bitwiseAND
        //     (Token::BitAnd, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // ^	BigInt	BigInt::bitwiseXOR
        //     (Token::BitXor, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     // |	BigInt	BigInt::bitwiseOR
        //     (Token::BitOr, JSValue::BigInt(lnum), JSValue::BigInt(rnum)) => todo!(),
        //     _ => unreachable!(),
        // };

        // 8. Return operation(lnum, rnum).
        todo!()
    }

    // 7. Let operation be the abstract operation associated with opText and Type(lnum) in the following table:
    // opText	Type(lnum)	operation
    let op_result = match (op_text, lnum, rnum) {
        // **	Number	Number::exponentiate
        (Token::Exponent, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.exponentiate(&rnum),
        // *	Number	Number::multiply
        (Token::Multiply, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.multiply(rnum),
        // /	Number	Number::divide
        (Token::Divide, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.divide(rnum),
        // %	Number	Number::remainder
        (Token::Modulo, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.remainder(rnum),
        // +	Number	Number::add
        (Token::Plus, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.add(rnum),
        // -	Number	Number::subtract
        (Token::Minus, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.subtract(rnum),
        // <<	Number	Number::leftShift
        (Token::LeftShift, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.left_shift(rnum),
        // >>	Number	Number::signedRightShift
        (Token::RightShift, JSValue::Number(lnum), JSValue::Number(rnum)) => {
            lnum.signed_right_shift(rnum)
        }
        // >>>	Number	Number::unsignedRightShift
        (Token::UnsignedRightShift, JSValue::Number(lnum), JSValue::Number(rnum)) => {
            lnum.unsigned_right_shift(rnum)
        }
        // &	Number	Number::bitwiseAND
        (Token::BitAnd, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.bitwise_and(rnum),
        // ^	Number	Number::bitwiseXOR
        (Token::BitXor, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.bitwise_xor(rnum),
        // |	Number	Number::bitwiseOR
        (Token::BitOr, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.bitwise_or(rnum),
        _ => unreachable!(),
    };

    // 8. Return operation(lnum, rnum).
    Ok(JSValue::Number(op_result))
}
