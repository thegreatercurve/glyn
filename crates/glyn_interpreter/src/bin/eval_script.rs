use glyn_interpreter::{
    eval_script, make_basic_object, JSAgent, JSObjectPropDescriptor, JSObjectPropKey, JSString,
    JSValue,
};

fn main() {
    let mut agent = JSAgent::default();

    let obj_1_handle = make_basic_object(&mut agent, vec![]);

    (agent.object_mut(obj_1_handle).methods.define_own_property)(
        &mut agent,
        obj_1_handle,
        &JSObjectPropKey::String(JSString::from("hello")),
        JSObjectPropDescriptor {
            value: Some(JSValue::String(JSString::from("world"))),
            ..Default::default()
        },
    )
    .unwrap();

    let obj_2_handle = make_basic_object(&mut agent, vec![]);

    (agent.object_mut(obj_2_handle).methods.set_prototype_of)(
        &mut agent,
        obj_2_handle,
        Some(obj_1_handle),
    );

    let value = (agent.object(obj_2_handle).methods.get)(
        &agent,
        obj_2_handle,
        &JSObjectPropKey::String(JSString::from("hello")),
        &JSValue::Undefined,
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
