use glyn_interpreter::{eval_script, JSAgent};
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_repl();

        return;
    }

    let i = 1;

    if i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help(&args[0]);
            }
            "--file" | "-f" => {
                if i + 1 < args.len() {
                    run_file(&args[i + 1]);
                } else {
                    eprintln!("Error: --file requires a filename argument");

                    print_help(&args[0]);

                    std::process::exit(1);
                }
            }
            "--eval" | "-e" => {
                if i + 1 < args.len() {
                    run_eval(&args[i + 1]);
                } else {
                    eprintln!("Error: --eval requires a JavaScript code string");

                    print_help(&args[0]);

                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Unknown argument '{}'", args[i]);

                print_help(&args[0]);

                std::process::exit(1);
            }
        }
    }
}

fn print_help(program_name: &str) {
    println!("Glyn JavaScript Interpreter");
    println!();
    println!("USAGE:");
    println!(
        "    {}                    Start interactive REPL",
        program_name
    );
    println!(
        "    {} --file <script>    Execute JavaScript file",
        program_name
    );
    println!(
        "    {} --eval <code>      Execute JavaScript code string",
        program_name
    );
    println!(
        "    {} --help             Show this help message",
        program_name
    );
    println!();
    println!("OPTIONS:");
    println!("    -f, --file <script>   Execute the specified JavaScript file");
    println!("    -e, --eval <code>     Execute the specified JavaScript code string");
    println!("    -h, --help            Print help information");
}

fn run_repl() {
    println!("Glyn JavaScript REPL");

    let mut agent = JSAgent::default();

    loop {
        print!("> ");

        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match eval_script(&mut agent, input) {
                    Ok(result) => println!("{:?}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);

                break;
            }
        }
    }
}

fn run_file(filename: &str) {
    let script_content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);

            std::process::exit(1);
        }
    };

    let mut agent = JSAgent::default();

    match eval_script(&mut agent, &script_content) {
        Ok(result) => println!("Result: {:?}", result),
        Err(err) => {
            eprintln!("Error evaluating script: {}", err);

            std::process::exit(1);
        }
    }
}

fn run_eval(code: &str) {
    let mut agent = JSAgent::default();

    match eval_script(&mut agent, code) {
        Ok(result) => println!("Result: {:?}", result),
        Err(err) => {
            eprintln!("Error evaluating code: {}", err);

            std::process::exit(1);
        }
    }
}
