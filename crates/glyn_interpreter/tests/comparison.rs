use glyn_interpreter::JSValue;

mod common;

#[test]
fn strict_equal() {
    assert_script_eq!("1 === 1", JSValue::Bool(true));
    assert_script_eq!("1 === 2", JSValue::Bool(false));
    assert_script_eq!("1 !== 2", JSValue::Bool(true));
    assert_script_eq!("1 !== 1", JSValue::Bool(false));
}
