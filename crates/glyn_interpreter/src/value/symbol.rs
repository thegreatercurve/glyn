/// 6.1.5 The Symbol Type
/// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-symbol-type
#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) struct JSSymbol {
    /// [[Description]]
    pub(crate) description: Option<String>,
}
