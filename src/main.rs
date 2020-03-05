use slide::{parse_arguments, run};
use std::process;

fn main() {
    let args = parse_arguments();
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
