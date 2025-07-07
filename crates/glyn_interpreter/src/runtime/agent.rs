use std::rc::Rc;

use safe_gc::Heap;

use crate::runtime::environment::Environment;
use crate::runtime::execution_context::ExecutionContext;
use crate::runtime::realm::Realm;
use crate::value::object::{JSObjAddr, JSObject};

#[derive(Debug)]
pub(crate) enum WellKnownSymbol {
    AsyncIterator,
    HasInstance,
    IsConcatSpreadable,
    Iterator,
    Match,
    MatchAll,
    Replace,
    Search,
    Species,
    Split,
    ToPrimitive,
    ToStringTag,
    Unscopables,
}

#[derive(Default)]
pub struct JSAgent {
    execution_contexts: Vec<ExecutionContext>,
    environment_records: Vec<Environment>,
    object_heap: Heap,
}

impl JSAgent {
    pub(crate) fn new() -> Self {
        Self {
            execution_contexts: vec![],
            environment_records: vec![],
            object_heap: Heap::new(),
        }
    }

    pub(crate) fn running_execution_context(&self) -> &ExecutionContext {
        debug_assert!(!self.execution_contexts.is_empty());

        // An execution context is a specification device that is used to track the runtime evaluation of code by an ECMAScript implementation. At any point in time, there is at most one execution context per agent that is actually executing code. This is known as the agent's running execution context.
        self.execution_contexts
            .last()
            .unwrap_or_else(|| unreachable!())
    }

    pub(crate) fn current_realm(&self) -> Rc<Realm> {
        self.running_execution_context().realm.clone()
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

    pub(crate) fn well_known_symbol(
        &self,
        obj_addr: JSObjAddr,
        symbol: WellKnownSymbol,
    ) -> Option<fn(agent: &JSAgent) -> Self> {
        let object = self.object(obj_addr);
        // Add a v-table look-up to check if object type has a well-known symbol.
        todo!()
    }
}
