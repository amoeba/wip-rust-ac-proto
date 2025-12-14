use std::env;

mod generate;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => generate::generate(),
        Some(cmd) => eprintln!("Unknown command: {}", cmd),
        None => eprintln!("Usage: cargo xtask generate"),
    }
}
