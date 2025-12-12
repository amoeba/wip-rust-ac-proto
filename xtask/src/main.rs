use std::{env, path::PathBuf};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => generate(),
        Some(cmd) => eprintln!("Unknown command: {}", cmd),
        None => eprintln!("Usage: cargo xtask generate"),
    }
}

fn generate() {
    // Get the workspace root
    let xtask_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = xtask_dir.parent().unwrap();
    let generated_dir = workspace_root.join("crates/acprotocol/src/generated");

    // Read FILTER_TYPES env var - comma-separated list of types to generate readers for
    let filter_types = env::var("FILTER_TYPES")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if !filter_types.is_empty() {
        println!("Generating readers for types: {:?}", filter_types);
    }

    // Use shared code generation workflow
    codegen::codegen::generate_and_write(&workspace_root, &generated_dir, &filter_types)
        .expect("Code generation failed");

    println!("Code generation complete!");
}
