/// 6.1.5 The Symbol Type
/// https://262.ecma-international.org/15.0/#sec-ecmascript-language-types-symbol-type
#[derive(Clone, Debug, PartialEq)]
pub struct JSSymbol {
    /// [[Description]]
    pub description: Option<String>,
}
