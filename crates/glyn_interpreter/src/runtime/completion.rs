use crate::JSValue;

/// 6.2.4 The Completion Record Specification Type
/// https://262.ecma-international.org/15.0/#sec-completion-record-specification-type
pub(crate) type CompletionRecord<T = NormalCompletion> = Result<T, ThrowCompletion>;

#[derive(Debug)]
pub(crate) struct NormalCompletion {
    pub(crate) value: JSValue,
    target: Option<JSValue>,
}

#[derive(Debug)]
pub(crate) struct ThrowCompletion {
    value: JSValue,
    target: Option<JSValue>,
}

/// 6.2.4.1 NormalCompletion ( value )
/// https://262.ecma-international.org/15.0/#sec-normalcompletion
pub(crate) fn normal_completion(value: JSValue) -> NormalCompletion {
    // 1. Return Completion Record { [[Type]]: normal, [[Value]]: value, [[Target]]: empty }.
    NormalCompletion {
        value,
        target: None,
    }
}

/// 6.2.4.2 ThrowCompletion ( value )
/// https://262.ecma-international.org/15.0/#sec-throwcompletion    
pub(crate) fn throw_completion(value: JSValue) -> ThrowCompletion {
    // 1. Return Completion Record { [[Type]]: throw, [[Value]]: value, [[Target]]: empty }.
    ThrowCompletion {
        value,
        target: None,
    }
}
