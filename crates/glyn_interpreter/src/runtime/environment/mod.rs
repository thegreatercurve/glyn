pub(crate) mod declarative_environment;
pub(crate) mod function_environment;
pub(crate) mod global_environment;
pub(crate) mod object_environment;

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
    value::{object::JSObjAddr, string::JSString, JSValue},
    JSAgent,
};

#[derive(Debug)]
pub(crate) enum EnvironmentKind {
    /// 9.1.1.1 Declarative Environment Records
    /// https://262.ecma-international.org/16.0/#sec-declarative-environment-records
    Declarative,

    /// 9.1.1.2 Object Environment Records
    /// https://262.ecma-international.org/16.0/#sec-object-environment-records
    Object,

    /// 9.1.1.3 Function Environment Records
    /// https://262.ecma-international.org/16.0/#sec-function-environment-records
    Function,

    /// 9.1.1.4 Global Environment Records
    /// https://262.ecma-international.org/16.0/#sec-global-environment-records
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

    /// InitializeBinding ( N, V )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) initialize_binding: fn(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
    ) -> CompletionRecord,

    /// SetMutableBinding ( N, V, S )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) set_mutable_binding: fn(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: JSString,
        value: JSValue,
        strict: bool,
    ) -> CompletionRecord,

    /// GetBindingValue ( N, S )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) get_binding_value: fn(
        agent: &JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
        strict: bool,
    ) -> CompletionRecord<JSValue>,

    /// DeleteBinding ( N )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) delete_binding: fn(
        agent: &mut JSAgent,
        env_addr: EnvironmentAddr,
        name: &JSString,
    ) -> CompletionRecord<bool>,

    /// HasThisBinding ( )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) has_this_binding: fn(agent: &JSAgent, env_addr: EnvironmentAddr) -> bool,

    /// HasSuperBinding ( )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) has_super_binding: fn(agent: &JSAgent, env_addr: EnvironmentAddr) -> bool,

    /// WithBaseObject ( )
    /// https://262.ecma-international.org/16.0/#table-abstract-methods-of-environment-records
    pub(crate) with_base_object:
        fn(agent: &JSAgent, env_addr: EnvironmentAddr) -> Option<JSObjAddr>,
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
        match kind {
            EnvironmentKind::Declarative => Self {
                outer: None,
                methods: &DECLARATIVE_ENVIRONMENT_METHODS,
                decl_env: Some(DeclEnvironment::default()),
                func_env: None,
                obj_env: None,
                global_env: None,
            },
            EnvironmentKind::Object => Self {
                outer: None,
                methods: &OBJECT_ENVIRONMENT_METHODS,
                decl_env: None,
                func_env: None,
                obj_env: Some(ObjEnvironment::default()),
                global_env: None,
            },
            EnvironmentKind::Function => Self {
                outer: None,
                methods: &FUNCTION_ENVIRONMENT_METHODS,
                decl_env: Some(DeclEnvironment::default()),
                func_env: Some(FuncEnvironment::default()),
                obj_env: None,
                global_env: None,
            },
            EnvironmentKind::Global => Self {
                outer: None,
                methods: &GLOBAL_ENVIRONMENT_METHODS,
                decl_env: Some(DeclEnvironment::default()),
                func_env: None,
                obj_env: Some(ObjEnvironment::default()),
                global_env: Some(GlobalEnvironment::default()),
            },
        }
    }

    pub(crate) fn decl_env(&self) -> &DeclEnvironment {
        self.decl_env.as_ref().unwrap()
    }

    pub(crate) fn decl_env_mut(&mut self) -> &mut DeclEnvironment {
        self.decl_env.as_mut().unwrap()
    }

    pub(crate) fn func_env(&self) -> &FuncEnvironment {
        self.func_env.as_ref().unwrap()
    }

    pub(crate) fn func_env_mut(&mut self) -> &mut FuncEnvironment {
        self.func_env.as_mut().unwrap()
    }

    pub(crate) fn obj_env(&self) -> &ObjEnvironment {
        self.obj_env.as_ref().unwrap()
    }

    pub(crate) fn obj_env_mut(&mut self) -> &mut ObjEnvironment {
        self.obj_env.as_mut().unwrap()
    }

    pub(crate) fn global_env(&self) -> &GlobalEnvironment {
        self.global_env.as_ref().unwrap()
    }

    pub(crate) fn global_env_mut(&mut self) -> &mut GlobalEnvironment {
        self.global_env.as_mut().unwrap()
    }
}
