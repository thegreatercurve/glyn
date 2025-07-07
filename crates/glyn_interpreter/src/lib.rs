mod abstract_ops;
mod codegen;
mod eval_script;
mod intrinsics;
mod lexer;
mod runtime;
mod value;
mod vm;

pub use eval_script::eval_script;
pub use runtime::agent::JSAgent;
