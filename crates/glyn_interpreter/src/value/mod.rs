use crate::value::{big_int::JSBigInt, number::JSNumber, string::JSString};

mod big_int;
mod number;
mod object;
mod string;
mod symbol;

pub(crate) use object::JSObject;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum JSValue {
    /// 6.1.1 The Undefined Type
    /// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-undefined-type
    Undefined,
    /// 6.1.2 The Null Type
    /// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-null-type
    Null,
    /// 6.1.3 The Boolean Type
    /// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-boolean-type
    Boolean(bool),
    String(JSString),
    Number(JSNumber),
    BigInt(JSBigInt),
    Symbol,
    Object(JSObject),
}
