use crate::JSValue;

/// 6.2.4 The Completion Record Specification Type
/// https://262.ecma-international.org/15.0/#sec-completion-record-specification-type
pub type CompletionRecord<T = NormalCompletion> = Result<T, ThrowCompletion>;

#[derive(Debug)]
pub struct NormalCompletion {
    pub value: JSValue,
    target: Option<JSValue>,
}

#[derive(Debug)]
pub struct ThrowCompletion {
    value: JSValue,
    target: Option<JSValue>,
}

/// 6.2.4.1 NormalCompletion ( value )
/// https://262.ecma-international.org/15.0/#sec-normalcompletion
pub(crate) fn normal_completion(value: JSValue) -> CompletionRecord {
    // 1. Return Completion Record { [[Type]]: normal, [[Value]]: value, [[Target]]: empty }.
    Ok(NormalCompletion {
        value,
        target: None,
    })
}

/// 6.2.4.2 ThrowCompletion ( value )
/// https://262.ecma-international.org/15.0/#sec-throwcompletion    
pub(crate) fn throw_completion(value: JSValue) -> CompletionRecord {
    // 1. Return Completion Record { [[Type]]: throw, [[Value]]: value, [[Target]]: empty }.
    Err(ThrowCompletion {
        value,
        target: None,
    })
}
