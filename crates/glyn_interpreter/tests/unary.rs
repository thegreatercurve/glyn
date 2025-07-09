use glyn_interpreter::JSValue;

mod common;

#[test]
fn unary_numbers() {
    assert_script_eq!("545", JSValue::Number(545.into()));
    assert_script_eq!("-545", JSValue::Number((-545).into()));
    assert_script_eq!("-+-523", JSValue::Number(523.into()));
}
