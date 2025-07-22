use crate::runtime::environment::Environment;
use crate::runtime::execution_context::ExecutionContext;
use crate::runtime::realm::RealmAddr;
use std::fmt::Display;

/// 6.1.5.1 Well-Known Symbols
/// https://262.ecma-international.org/16.0/#sec-well-known-symbols
#[derive(Debug)]
pub(crate) enum WellKnownSymbols {
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

impl Display for WellKnownSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{self:?}%")
    }
}

pub(crate) const WELL_KNOWN_SYMBOLS_ASYNC_ITERATOR: WellKnownSymbols =
    WellKnownSymbols::AsyncIterator;
pub(crate) const WELL_KNOWN_SYMBOLS_HAS_INSTANCE: WellKnownSymbols = WellKnownSymbols::HasInstance;
pub(crate) const WELL_KNOWN_SYMBOLS_IS_CONCAT_SPREADABLE: WellKnownSymbols =
    WellKnownSymbols::IsConcatSpreadable;
pub(crate) const WELL_KNOWN_SYMBOLS_ITERATOR: WellKnownSymbols = WellKnownSymbols::Iterator;
pub(crate) const WELL_KNOWN_SYMBOLS_MATCH: WellKnownSymbols = WellKnownSymbols::Match;
pub(crate) const WELL_KNOWN_SYMBOLS_MATCH_ALL: WellKnownSymbols = WellKnownSymbols::MatchAll;
pub(crate) const WELL_KNOWN_SYMBOLS_REPLACE: WellKnownSymbols = WellKnownSymbols::Replace;
pub(crate) const WELL_KNOWN_SYMBOLS_SEARCH: WellKnownSymbols = WellKnownSymbols::Search;
pub(crate) const WELL_KNOWN_SYMBOLS_SPECIES: WellKnownSymbols = WellKnownSymbols::Species;
pub(crate) const WELL_KNOWN_SYMBOLS_SPLIT: WellKnownSymbols = WellKnownSymbols::Split;
pub(crate) const WELL_KNOWN_SYMBOLS_TO_PRIMITIVE: WellKnownSymbols = WellKnownSymbols::ToPrimitive;
pub(crate) const WELL_KNOWN_SYMBOLS_TO_STRING_TAG: WellKnownSymbols = WellKnownSymbols::ToStringTag;
pub(crate) const WELL_KNOWN_SYMBOLS_UNSCOPABLES: WellKnownSymbols = WellKnownSymbols::Unscopables;

#[derive(Default)]
pub struct JSAgent {
    pub(crate) execution_contexts: Vec<ExecutionContext>,
    environment_records: Vec<Environment>,
}

impl JSAgent {
    pub(crate) fn new() -> Self {
        Self {
            execution_contexts: vec![],
            environment_records: vec![],
        }
    }

    pub(crate) fn running_execution_context(&self) -> &ExecutionContext {
        debug_assert!(!self.execution_contexts.is_empty());

        // An execution context is a specification device that is used to track the runtime evaluation of code by an ECMAScript implementation. At any point in time, there is at most one execution context per agent that is actually executing code. This is known as the agent's running execution context.
        self.execution_contexts.last().unwrap()
    }

    pub(crate) fn current_realm(&self) -> RealmAddr {
        self.running_execution_context().realm.clone()
    }

    pub(crate) fn push_execution_context(&mut self, context: ExecutionContext) {
        self.execution_contexts.push(context);
    }

    pub(crate) fn pop_execution_context(&mut self) -> ExecutionContext {
        self.execution_contexts.pop().unwrap()
    }
}

pub(crate) fn type_error(message: &str) -> ! {
    panic!("TypeError: {message:?}");
}

pub(crate) fn reference_error(message: &str) -> ! {
    panic!("ReferenceError: {message:?}");
}

pub(crate) fn syntax_error(message: &str) -> ! {
    panic!("SyntaxError: {message:?}");
}

pub(crate) fn range_error(message: &str) -> ! {
    panic!("RangeError: {message:?}");
}
