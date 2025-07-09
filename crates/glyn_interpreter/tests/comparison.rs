use glyn_interpreter::JSValue;

mod common;

#[test]
fn strictly_equal() {
    assert_script_eq!("1 === 1", JSValue::Bool(true));
    assert_script_eq!("1 === 2", JSValue::Bool(false));
    assert_script_eq!("1 !== 2", JSValue::Bool(true));
    assert_script_eq!("1 !== 1", JSValue::Bool(false));
    assert_script_eq!("-1 === 1", JSValue::Bool(false));
    assert_script_eq!("-0 === +0", JSValue::Bool(true));
    assert_script_eq!("+0 === -0", JSValue::Bool(true));
}

#[test]
fn loosely_equal() {
    assert_script_eq!("1 == 1", JSValue::Bool(true));
    assert_script_eq!("1 == 2", JSValue::Bool(false));
    assert_script_eq!("1 != 2", JSValue::Bool(true));
    assert_script_eq!("1 != 1", JSValue::Bool(false));
    assert_script_eq!("-1 == 1", JSValue::Bool(false));
    assert_script_eq!("-0 == +0", JSValue::Bool(true));
    assert_script_eq!("+0 == -0", JSValue::Bool(true));
    // TODO: Add more tests for different types.
}
