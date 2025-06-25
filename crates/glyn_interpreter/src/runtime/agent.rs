use crate::{runtime::Environment, Realm};

#[derive(Debug, Default)]
pub struct ExecutionContext;

#[derive(Debug, Default)]
pub struct JSAgent {
    execution_contexts: Vec<ExecutionContext>,
    environment_records: Vec<Environment>,
}

impl JSAgent {
    pub fn new(realm: Realm) -> Self {
        Self {
            execution_contexts: vec![ExecutionContext::default()],
            environment_records: vec![],
        }
    }
}

impl JSAgent {
    fn running_execution_context(&self) -> &ExecutionContext {
        debug_assert!(!self.execution_contexts.is_empty());

        // An execution context is a specification device that is used to track the runtime evaluation of code by an ECMAScript implementation. At any point in time, there is at most one execution context per agent that is actually executing code. This is known as the agent's running execution context.
        self.execution_contexts.last().unwrap_or_else(|| {
            unreachable!("Expected at least one execution context to be running.")
        })
    }
}
