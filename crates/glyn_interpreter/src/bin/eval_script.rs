use glyn_interpreter::{eval_script, JSAgent};

fn main() {
    let mut agent = JSAgent::default();

    let script_str = r#"
        let x = 10;
        x + 5;
    "#;

    let result = eval_script(&mut agent, script_str).unwrap_or_else(|err| {
        panic!("Error evaluating script: {err:?}");
    });

    println!("Result: {result:?}");
}
