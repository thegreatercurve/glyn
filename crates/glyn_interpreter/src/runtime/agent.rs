use safe_gc::{Gc, Heap};

use crate::{runtime::Environment, JSObject};

#[derive(Debug, Default)]
pub struct ExecutionContext;

#[derive(Default)]
pub struct JSAgent {
    pub(crate) execution_contexts: Vec<ExecutionContext>,
    pub(crate) environment_records: Vec<Environment>,
    pub object_heap: Heap,
}

impl JSAgent {
    pub fn new() -> Self {
        Self {
            execution_contexts: vec![ExecutionContext],
            environment_records: vec![],
            object_heap: Heap::new(),
        }
    }

    pub(crate) fn running_execution_context(&self) -> &ExecutionContext {
        debug_assert!(!self.execution_contexts.is_empty());

        // An execution context is a specification device that is used to track the runtime evaluation of code by an ECMAScript implementation. At any point in time, there is at most one execution context per agent that is actually executing code. This is known as the agent's running execution context.
        self.execution_contexts.last().unwrap_or_else(|| {
            unreachable!("Expected at least one execution context to be running.")
        })
    }

    pub(crate) fn push_execution_context(&mut self, context: ExecutionContext) {
        self.execution_contexts.push(context);
    }

    pub(crate) fn pop_execution_context(&mut self) -> ExecutionContext {
        self.execution_contexts.pop().unwrap_or_else(|| {
            unreachable!("Expected at least one execution context to be running.")
        })
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

    pub fn get_object(&self, object: Gc<JSObject>) -> &JSObject {
        self.object_heap.get(object)
    }

    pub fn allocate_object(&mut self, object: JSObject) -> Gc<JSObject> {
        self.object_heap.alloc(object).into()
    }
}
