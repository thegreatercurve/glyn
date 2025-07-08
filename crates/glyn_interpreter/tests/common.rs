#[macro_export]
macro_rules! assert_script_eq {
    ($source: expr, $expected: expr) => {
        let mut agent = glyn_interpreter::JSAgent::default();

        let completion_record = glyn_interpreter::eval_script(&mut agent, $source);

        match completion_record {
            Ok(result) => assert_eq!(result, $expected),
            Err(err) => panic!("Error evaluating script: {err:?}"),
        }
    };
}
