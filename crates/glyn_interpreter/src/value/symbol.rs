use crate::runtime::completion::{throw_completion, ThrowCompletion};
use crate::value::JSValue;

/// 6.1.5 The Symbol Type
/// https://262.ecma-international.org/16.0/#sec-ecmascript-language-types-symbol-type
#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) struct JSSymbol {
    /// [[Description]]
    pub(crate) description: Option<String>,
}

impl From<String> for JSSymbol {
    fn from(value: String) -> Self {
        Self {
            description: Some(value),
        }
    }
}

impl TryFrom<JSValue> for JSSymbol {
    type Error = ThrowCompletion;

    fn try_from(value: JSValue) -> Result<Self, Self::Error> {
        match value {
            JSValue::Symbol(symbol) => Ok(symbol),
            _ => throw_completion("Expected JSValue::Symbol for conversion to JSSymbol"),
        }
    }
}

impl TryFrom<&JSValue> for JSSymbol {
    type Error = ThrowCompletion;

    fn try_from(value: &JSValue) -> Result<Self, Self::Error> {
        match value {
            JSValue::Symbol(symbol) => Ok(symbol.clone()),
            _ => throw_completion("Expected JSValue::Symbol for conversion to JSSymbol"),
        }
    }
}
