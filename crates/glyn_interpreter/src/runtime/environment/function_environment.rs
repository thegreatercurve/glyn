use crate::{
    runtime::environment::{declarative_environment::DeclEnvironment, EnvironmentMethods},
    value::object::JSObjAddr,
    JSValue,
};

#[derive(Debug, Default)]
pub(crate) enum ThisBindingStatus {
    Lexical,
    Initialized,
    #[default]
    Uninitialized,
}

/// 9.1.1.2 Function Environment Records
/// https://262.ecma-international.org/16.0/#sec-function-environment-records
#[derive(Debug, Default)]
pub(crate) struct FuncEnvironment {
    /// [[ThisValue]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    this_value: Option<JSValue>,

    /// [[ThisBindingStatus]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    this_binding_status: ThisBindingStatus,

    /// [[FunctionObject]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    function_object: Option<JSObjAddr>,

    /// [[NewTarget]]
    /// https://262.ecma-international.org/16.0/#table-additional-fields-of-function-environment-records
    new_target: Option<JSValue>,
}

pub(crate) static FUNCTION_ENVIRONMENT_METHODS: EnvironmentMethods = EnvironmentMethods {
    has_binding: DeclEnvironment::has_binding,
    create_mutable_binding: DeclEnvironment::create_mutable_binding,
    create_immutable_binding: DeclEnvironment::create_immutable_binding,
};
