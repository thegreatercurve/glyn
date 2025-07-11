use crate::{
    runtime::environment::EnvironmentAddr,
    value::{string::JSString, JSValue},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ReferenceBase {
    Value(JSValue),
    Environment(EnvironmentAddr),
    Unresolvable,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ReferenceName {
    Value(JSValue),
    PrivateName(String),
}

impl From<JSString> for ReferenceName {
    fn from(value: JSString) -> Self {
        ReferenceName::Value(JSValue::String(value))
    }
}

impl From<&JSString> for ReferenceName {
    fn from(value: &JSString) -> Self {
        ReferenceName::Value(JSValue::String(value.clone()))
    }
}

/// 6.2.5 The Reference Record Specification Type
/// https://262.ecma-international.org/16.0/#sec-reference-record-specification-type
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Reference {
    /// [[Base]]
    pub(crate) base: ReferenceBase,

    /// [[ReferencedName]]
    pub(crate) referenced_name: ReferenceName,

    /// [[Strict]]
    pub(crate) strict: bool,

    /// [[ThisValue]]
    pub(crate) this_value: Option<JSValue>,
}
