use glyn_interpreter::{
    eval_script, make_basic_object, JSAgent, JSObjectPropDescriptor, JSObjectPropKey, JSString,
    JSValue,
};

fn main() {
    let mut agent = JSAgent::default();

    let mut obj1 = make_basic_object(&[]);

    (obj1.methods.define_own_property)(
        &mut agent,
        &mut obj1,
        &JSObjectPropKey::String(JSString::from("hello")),
        JSObjectPropDescriptor {
            value: Some(JSValue::String(JSString::from("world"))),
            ..Default::default()
        },
    );

    let mut obj2 = make_basic_object(&[]);

    let obj_1_handle = agent.allocate_object(obj1);

    (obj2.methods.set_prototype_of)(&mut agent, &mut obj2, Some(obj_1_handle));

    let value = (obj2.methods.get)(
        &agent,
        &obj2,
        &JSObjectPropKey::String(JSString::from("hello")),
        None,
    );

    println!("value: {:?}", value);

    let script_str = r#"
        let x = 10;
        x + 5;
    "#;

    let result = eval_script(&mut agent, script_str).unwrap_or_else(|err| {
        panic!("Error evaluating script: {err:?}");
    });

    println!("Result: {result:?}");
}
