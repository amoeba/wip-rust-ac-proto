/// Represents a generated file with its path and content
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}

/// Represents the collection of generated files
pub struct GeneratedCode {
    pub files: Vec<GeneratedFile>,
}
