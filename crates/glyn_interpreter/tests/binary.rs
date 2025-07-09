use glyn_interpreter::JSValue;

mod common;

#[test]
fn arithmetic() {
    assert_script_eq!("5 + 5", JSValue::Number(10.into()));
    assert_script_eq!("5 - 5", JSValue::Number(0.into()));
    assert_script_eq!("5 * 5", JSValue::Number(25.into()));
    assert_script_eq!("6 / 2", JSValue::Number(3.into()));
    assert_script_eq!("5 % 2", JSValue::Number(1.into()));
    assert_script_eq!("5 ** 2", JSValue::Number(25.into()));
}

#[test]
fn bitwise_arithmetic() {
    assert_script_eq!("2 & 3", JSValue::Number(2.into()));
    assert_script_eq!("5 | 3", JSValue::Number(7.into()));
    assert_script_eq!("5 ^ 3", JSValue::Number(6.into()));
    assert_script_eq!("5 << 1", JSValue::Number(10.into()));
    assert_script_eq!("5 >> 1", JSValue::Number(2.into()));
    assert_script_eq!("5 >>> 1", JSValue::Number(2.into()));
    assert_script_eq!("5 >>> 1", JSValue::Number(2.into()));
    assert_script_eq!("3 << 4 >> 3", JSValue::Number(6.into()));
}

#[test]
fn operator_precedence() {
    assert_script_eq!("5 + 4 * 6", JSValue::Number(29.into()));
    assert_script_eq!("4 * 6 + 5", JSValue::Number(29.into()));
    assert_script_eq!("4 * 5 / 2 * 3", JSValue::Number(30.into()));
    assert_script_eq!("5 + 5 ** 4 * 4", JSValue::Number(2505.into()));
    assert_script_eq!("2 ** 2 ** 3", JSValue::Number(256.into()));
    assert_script_eq!("2 << 7 >> 1 ** 5 / 2", JSValue::Number(256.into()));
}

#[test]
fn relational() {
    assert_script_eq!("3 > 2", JSValue::Bool(true));
    assert_script_eq!("3 > 3", JSValue::Bool(false));
    assert_script_eq!("3 >= 2", JSValue::Bool(true));
    assert_script_eq!("3 >= 3", JSValue::Bool(true));
    assert_script_eq!("3 < 4", JSValue::Bool(true));
    assert_script_eq!("4 < 4", JSValue::Bool(false));
    assert_script_eq!("4 <= 5", JSValue::Bool(true));
    assert_script_eq!("4 <= 4", JSValue::Bool(true));
}
