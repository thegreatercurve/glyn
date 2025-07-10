mod declarative_environment;
mod function_environment;
mod global_environment;
mod object_environment;

use safe_gc::{Collector, Gc, Trace};

use crate::{
    runtime::{
        completion::CompletionRecord,
        environment::{
            declarative_environment::{DeclEnvironment, DECLARATIVE_ENVIRONMENT_METHODS},
            function_environment::{FuncEnvironment, FUNCTION_ENVIRONMENT_METHODS},
            global_environment::{GlobalEnvironment, GLOBAL_ENVIRONMENT_METHODS},
            object_environment::{ObjEnvironment, OBJECT_ENVIRONMENT_METHODS},
        },
    },
    value::string::JSString,
    JSAgent,
};

#[derive(Debug)]
pub(crate) enum EnvironmentKind {
    Declarative,
    Object,
    Function,
    Global,
}

#[derive(Debug)]
pub(crate) struct EnvironmentMethods {
    /// HasBinding ( N )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) has_binding:
        fn(agent: &JSAgent, env_addr: EnvironmentAddr, name: &JSString) -> CompletionRecord<bool>,

    /// CreateMutableBinding ( N, D )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) create_mutable_binding: fn(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        deletable: bool,
    ) -> CompletionRecord,

    /// CreateImmutableBinding ( N, S )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) create_immutable_binding: fn(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        strict: bool,
    ) -> CompletionRecord,
}

pub(crate) type EnvironmentAddr = Gc<Environment>;

/// 9.1.1 The Environment Record Type Hierarchy
/// https://262.ecma-international.org/16.0/#sec-the-environment-record-type-hierarchy
#[derive(Debug)]
pub(crate) struct Environment {
    /// [[OuterEnv]]
    pub(crate) outer: Option<EnvironmentAddr>,
    pub(crate) methods: &'static EnvironmentMethods,

    decl_env: Option<DeclEnvironment>,
    func_env: Option<FuncEnvironment>,
    obj_env: Option<ObjEnvironment>,
    global_env: Option<GlobalEnvironment>,
}

impl Trace for Environment {
    fn trace(&self, _collector: &mut Collector) {}
}

impl Environment {
    pub(crate) fn new(kind: EnvironmentKind) -> Self {
        Self {
            outer: None,
            methods: match kind {
                EnvironmentKind::Declarative => &DECLARATIVE_ENVIRONMENT_METHODS,
                EnvironmentKind::Object => &OBJECT_ENVIRONMENT_METHODS,
                EnvironmentKind::Function => &FUNCTION_ENVIRONMENT_METHODS,
                EnvironmentKind::Global => &GLOBAL_ENVIRONMENT_METHODS,
            },
            decl_env: None,
            func_env: None,
            obj_env: None,
            global_env: None,
        }
    }
}
