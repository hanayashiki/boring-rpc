mod codegen;

use std::env;

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("codegen") => codegen::generate_syn(),
        _ => eprintln!("Unknown task"),
    }
}
