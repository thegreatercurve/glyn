use glyn_interpreter::{
    eval_script, make_basic_object, JSAgent, JSObjectPropDescriptor, JSObjectPropKey, JSString,
    JSValue,
};

fn main() {
    let mut agent = JSAgent::default();

    let mut obj1 = make_basic_object(&[]);

    (obj1.methods.define_own_property)(
        &mut obj1,
        &JSObjectPropKey::String(JSString::from("hello")),
        JSObjectPropDescriptor {
            value: Some(JSValue::String(JSString::from("world"))),
            ..Default::default()
        },
    );

    let mut obj2 = make_basic_object(&[]);

    (obj2.methods.set_prototype_of)(&mut obj2, Some(&mut obj1));

    let value = (obj2.methods.get)(
        &agent,
        &obj2,
        &JSObjectPropKey::String(JSString::from("hello")),
    );

    println!("value: {value:?}");

    let script_str = r#"
        let x = 10;
        x + 5;
    "#;

    let result = eval_script(&mut agent, script_str).unwrap_or_else(|err| {
        panic!("Error evaluating script: {err:?}");
    });

    println!("Result: {result:?}");
}
