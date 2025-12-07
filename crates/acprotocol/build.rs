use std::{env, fs, path::PathBuf};

fn main() {
    env_logger::init();

    // Get the workspace root (two levels up from the gen crate)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let protocol_path = workspace_root.join("ACProtocol/protocol.xml");

    let generated_dir = manifest_dir.join("src/generated");

    // Clean the generated directory to remove old structure
    if generated_dir.exists() {
        fs::remove_dir_all(&generated_dir).unwrap();
    }

    // Commented out for testing
    // println!("cargo:rerun-if-changed={}", protocol_path.display());

    // Read FILTER_TYPES env var - comma-separated list of types to generate readers for
    let filter_types = env::var("FILTER_TYPES")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if !filter_types.is_empty() {
        println!(
            "cargo:warning=Generating readers for types: {:?}",
            filter_types
        );
    }

    let xml = fs::read_to_string(&protocol_path).unwrap();
    let generated_code = genlib::generate(&xml, &filter_types);

    // Write all generated files
    for file in generated_code.files {
        let file_path = generated_dir.join(&file.path);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(&file_path, file.content).unwrap();
    }
}
