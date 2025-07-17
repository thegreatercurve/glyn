use crate::gc::{Gc, Trace, Tracer};
use crate::runtime::environment::EnvironmentAddr;
use crate::runtime::intrinsics::Intrinsics;
use crate::value::object::JSObjAddr;

pub(crate) type RealmAddr = Gc<Realm>;

/// 9.3 Realms
/// https://262.ecma-international.org/16.0/#sec-code-realms
#[derive(Debug, Default)]
pub(crate) struct Realm {
    /// [[Intrinsics]]
    pub(crate) intrinsics: Intrinsics,

    /// [[GlobalObject]]
    pub(crate) global_object: Option<JSObjAddr>,

    /// [[GlobalEnv]]
    pub(crate) global_env: Option<EnvironmentAddr>,
}

impl Trace for Realm {
    fn trace(&self, tracer: &mut Tracer) {
        if let Some(global_object) = self.global_object {
            tracer.edge(global_object);
        }

        if let Some(global_env) = self.global_env {
            tracer.edge(global_env);
        }
    }
}
