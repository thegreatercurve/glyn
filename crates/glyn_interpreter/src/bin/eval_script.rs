use glyn_interpreter::{eval_script, JSAgent};

fn main() {
    let mut agent = JSAgent::default();

    let result = eval_script(&mut agent, "5 + 4 * 2").unwrap_or_else(|err| {
        panic!("Error evaluating script: {err:?}");
    });

    println!("Result: {result:?}");
}
