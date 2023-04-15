use std::vec;

use serde::Deserialize;

const MANIFEST: &str = env!("CARGO_MANIFEST_DIR");

macro_rules! p {
    ($($tokens: tt)*) => {
        #[cfg(feature="debug")]
        println!("cargo:warning={}", format!($($tokens)*));
    }
}

fn project_root() -> &'static std::path::Path {
    std::path::Path::new(MANIFEST).ancestors().nth(1).unwrap()
}

#[derive(Debug, Deserialize)]
struct Punctuation {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct SyntaxKind {
    punctuation: Vec<Punctuation>,
}

macro_rules! generate_syntax_enum {
    ($elem:expr) => {
        "ok"
    };
}

fn main() {
    let project_root = project_root();
    p!("Project root {:?}", project_root);

    let syntax_path = project_root.join("syntax/token_config.jsonc");

    let syntax = std::fs::read_to_string(syntax_path).expect("Could not read syntax file");

    // split syntax into lines
    let syntax: Vec<&str> = syntax.lines().collect();

    // filter out comments
    let syntax: Vec<&str> = syntax
        .into_iter()
        .filter(|line| !line.trim().starts_with("//"))
        .collect();

    // join
    let syntax: String = syntax.join("\n");

    let syntax = serde_json::from_str::<SyntaxKind>(&syntax).expect("Could not parse syntax file");

    p!("syntax: {:?}", syntax.punctuation);

    let x = generate_syntax_enum!(syntax.punctuation);
}
