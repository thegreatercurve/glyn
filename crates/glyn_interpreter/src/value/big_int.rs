/// 6.1.8 The BigInt Type
/// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-bigint-type
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct JSBigInt;

impl JSBigInt {
    pub(crate) fn is_zero(&self) -> bool {
        false
    }
}
