use glyn_interpreter::{eval_script, JSAgent};

fn main() {
    let mut agent = JSAgent::default();

    let result = eval_script(&mut agent, r"3 + 5 * 3 / 2 ** 2 + 1").unwrap_or_else(|err| {
        panic!("Error evaluating script: {err:?}");
    });

    println!("Result: {result:?}");
}
