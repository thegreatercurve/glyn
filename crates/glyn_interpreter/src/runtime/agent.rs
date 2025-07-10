use safe_gc::Heap;

use crate::runtime::environment::{Environment, EnvironmentAddr};
use crate::runtime::execution_context::ExecutionContext;
use crate::runtime::realm::{Realm, RealmAddr};
use crate::value::object::{JSObjAddr, JSObject};
use crate::value::symbol::JSSymbol;

/// 6.1.5.1 Well-Known Symbols
/// https://262.ecma-international.org/16.0/#sec-well-known-symbols

#[derive(Default)]
pub(crate) struct WellKnownSymbols {
    pub(crate) async_iterator: JSSymbol,
    pub(crate) has_instance: JSSymbol,
    pub(crate) is_concat_spreadable: JSSymbol,
    pub(crate) iterator: JSSymbol,
    pub(crate) match_: JSSymbol,
    pub(crate) match_all: JSSymbol,
    pub(crate) replace: JSSymbol,
    pub(crate) search: JSSymbol,
    pub(crate) species: JSSymbol,
    pub(crate) split: JSSymbol,
    pub(crate) to_primitive: JSSymbol,
    pub(crate) to_string_tag: JSSymbol,
    pub(crate) unscopables: JSSymbol,
}

#[derive(Default)]
pub struct JSAgent {
    execution_contexts: Vec<ExecutionContext>,
    environment_records: Vec<Environment>,
    object_heap: Heap,
    realm_heap: Heap,
    environment_heap: Heap,
    well_known_symbols: WellKnownSymbols,
}

impl JSAgent {
    pub(crate) fn new() -> Self {
        Self {
            execution_contexts: vec![],
            environment_records: vec![],
            object_heap: Heap::new(),
            realm_heap: Heap::new(),
            environment_heap: Heap::new(),
            well_known_symbols: WellKnownSymbols::default(),
        }
    }

    pub(crate) fn running_execution_context(&self) -> &ExecutionContext {
        debug_assert!(!self.execution_contexts.is_empty());

        // An execution context is a specification device that is used to track the runtime evaluation of code by an ECMAScript implementation. At any point in time, there is at most one execution context per agent that is actually executing code. This is known as the agent's running execution context.
        self.execution_contexts
            .last()
            .unwrap_or_else(|| unreachable!())
    }

    pub(crate) fn current_realm(&self) -> RealmAddr {
        self.running_execution_context().realm
    }

    pub(crate) fn push_execution_context(&mut self, context: ExecutionContext) {
        self.execution_contexts.push(context);
    }

    pub(crate) fn pop_execution_context(&mut self) -> ExecutionContext {
        self.execution_contexts
            .pop()
            .unwrap_or_else(|| unreachable!())
    }

    pub(crate) fn type_error(&self, message: &str) -> ! {
        panic!("TypeError: {message:?}");
    }

    pub(crate) fn reference_error(&self, message: &str) -> ! {
        panic!("ReferenceError: {message:?}");
    }

    pub(crate) fn syntax_error(&self, message: &str) -> ! {
        panic!("SyntaxError: {message:?}");
    }

    pub(crate) fn range_error(&self, message: &str) -> ! {
        panic!("RangeError: {message:?}");
    }

    pub(crate) fn allocate_object(&mut self, object: JSObject) -> JSObjAddr {
        self.object_heap.alloc(object).into()
    }

    pub(crate) fn object(&self, obj_addr: JSObjAddr) -> &JSObject {
        self.object_heap.get(obj_addr)
    }

    pub(crate) fn object_mut(&mut self, obj_addr: JSObjAddr) -> &mut JSObject {
        self.object_heap.get_mut(obj_addr)
    }

    pub(crate) fn allocate_realm(&mut self, realm: Realm) -> RealmAddr {
        self.realm_heap.alloc(realm).into()
    }

    pub(crate) fn realm(&self, realm_addr: RealmAddr) -> &Realm {
        self.realm_heap.get(realm_addr)
    }

    pub(crate) fn realm_mut(&mut self, realm_addr: RealmAddr) -> &mut Realm {
        self.realm_heap.get_mut(realm_addr)
    }

    pub(crate) fn allocate_environment(&mut self, environment: Environment) -> EnvironmentAddr {
        self.environment_heap.alloc(environment).into()
    }

    pub(crate) fn environment(&self, environment_addr: EnvironmentAddr) -> &Environment {
        self.environment_heap.get(environment_addr)
    }

    pub(crate) fn environment_mut(
        &mut self,
        environment_addr: EnvironmentAddr,
    ) -> &mut Environment {
        self.environment_heap.get_mut(environment_addr)
    }

    pub(crate) fn well_known_symbols(&self) -> &WellKnownSymbols {
        &self.well_known_symbols
    }
}
