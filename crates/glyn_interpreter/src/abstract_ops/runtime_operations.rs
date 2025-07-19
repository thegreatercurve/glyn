use crate::{
    abstract_ops::{
        testing_comparison::same_type,
        type_conversion::{to_numeric, to_primitive, to_string, PreferredPrimType},
    },
    lexer::Token,
    runtime::{agent::type_error, completion::CompletionRecord},
    value::{string::JSString, JSValue},
};

/// 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
/// https://262.ecma-international.org/16.0/#sec-applystringornumericbinaryoperator
pub(crate) fn apply_string_or_numeric_binary_operator(
    lval: JSValue,
    rval: JSValue,
) -> CompletionRecord<JSValue> {
    // 1. If opText is +, then
    // NOTE: Implemented in the VM.

    // a. Let lprim be ? ToPrimitive(lval).
    let lprim = to_primitive(lval, PreferredPrimType::Default)?;

    // b. Let rprim be ? ToPrimitive(rval).
    let rprim = to_primitive(rval, PreferredPrimType::Default)?;

    // c. If lprim is a String or rprim is a String, then
    if lprim.is_string() || rprim.is_string() {
        // i. Let lstr be ? ToString(lprim).
        let lstr = to_string(lprim)?;

        // ii. Let rstr be ? ToString(rprim).
        let rstr = to_string(rprim)?;

        // iii. Return the string-concatenation of lstr and rstr.
        return Ok(JSValue::String(JSString::from(lstr.0 + &rstr.0)));
    }

    // d. Set lval to lprim.
    // e. Set rval to rprim.
    apply_numeric_binary_operator(lprim, Token::Plus, rprim)
}

/// 13.15.3 ApplyStringOrNumericBinaryOperator ( lval, opText, rval )
/// https://262.ecma-international.org/16.0/#sec-applystringornumericbinaryoperator
pub(crate) fn apply_numeric_binary_operator(
    lval: JSValue,
    op_text: Token,
    rval: JSValue,
) -> CompletionRecord<JSValue> {
    // 2. NOTE: At this point, it must be a numeric operation.
    // 3. Let lnum be ? ToNumeric(lval).
    let lnum = to_numeric(lval)?;

    // 4. Let rnum be ? ToNumeric(rval).
    let rnum = to_numeric(rval)?;

    // 5. If SameType(lNum, rNum) is false, throw a TypeError exception.
    if !same_type(&lnum, &rnum) {
        type_error(&format!(
            "Cannot use {:?} and {:?} in a binary expression",
            lnum, rnum
        ));
    }

    // 6. If lNum is a BigInt, then
    if lnum.is_big_int() {
        // a. If opText is **, return ? BigInt::exponentiate(lnum, rnum).
        // b. If opText is /, return ? BigInt::divide(lnum, rnum).
        // c. If opText is %, return ? BigInt::remainder(lnum, rnum).
        // d. If opText is >>>, return ? BigInt::unsignedRightShift(lnum, rnum).
        // e. Let operation be the abstract operation associated with opText in the following table:

        // opText	operation
        // *	BigInt::multiply
        // +	BigInt::add
        // -	BigInt::subtract
        // <<	BigInt::leftShift
        // >>	BigInt::signedRightShift
        // &	BigInt::bitwiseAND
        // ^	BigInt::bitwiseXOR
        // |	BigInt::bitwiseOR

        // 8. Return operation(lNum, rNum).
        todo!()
    } else {
        // a. Assert: lNum is a Number.
        // b. Let operation be the abstract operation associated with opText in the following table:
        // opText	operation
        let op_result = match (op_text, lnum, rnum) {
            // **	Number::exponentiate
            (Token::Exponent, JSValue::Number(lnum), JSValue::Number(rnum)) => {
                lnum.exponentiate(&rnum)
            }
            // *	Number::multiply
            (Token::Multiply, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.multiply(rnum),
            // /	Number::divide
            (Token::Divide, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.divide(rnum),
            // %	Number::remainder
            (Token::Modulo, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.remainder(rnum),
            // +	Number::add
            (Token::Plus, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.add(rnum),
            // -	Number::subtract
            (Token::Minus, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.subtract(rnum),
            // <<	Number::leftShift
            (Token::LeftShift, JSValue::Number(lnum), JSValue::Number(rnum)) => {
                lnum.left_shift(rnum)
            }
            // >>	Number::signedRightShift
            (Token::RightShift, JSValue::Number(lnum), JSValue::Number(rnum)) => {
                lnum.signed_right_shift(rnum)
            }
            // >>>	Number::unsignedRightShift
            (Token::UnsignedRightShift, JSValue::Number(lnum), JSValue::Number(rnum)) => {
                lnum.unsigned_right_shift(rnum)
            }
            // &	Number::bitwiseAND
            (Token::BitAnd, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.bitwise_and(rnum),
            // ^	Number::bitwiseXOR
            (Token::BitXor, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.bitwise_xor(rnum),
            // |	Number::bitwiseOR
            (Token::BitOr, JSValue::Number(lnum), JSValue::Number(rnum)) => lnum.bitwise_or(rnum),
            _ => unreachable!(),
        };

        // 8. Return operation(lNum, rNum).
        Ok(JSValue::Number(op_result))
    }
}
