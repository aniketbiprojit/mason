use convert_case::{Case, Casing};
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

    let mut out_stream = format!(
        "{}\n{}",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]", "pub enum SyntaxKind {"
    );

    for token in &syntax.punctuation {
        out_stream.push_str(&format!("\n    {},", token.name.to_case(Case::UpperCamel)));
    }

    out_stream.push_str("\n}");

    out_stream.push_str("\n\npub fn match_token(token: &str) -> Option<SyntaxKind> {");
    out_stream.push_str("\n    match token {");
    // create match arm for each token
    for token in &syntax.punctuation {
        out_stream.push_str(&format!(
            "\n        \"{}\" => Some(SyntaxKind::{}),",
            token.value,
            token.name.to_case(Case::UpperCamel)
        ));
    }
    out_stream.push_str(&format!("\n        {} => {},", "_", "None"));

    out_stream.push_str("\n    }");
    out_stream.push_str("\n}");

    let out_dir =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not get OUT_DIR"));

    let out_file = out_dir.join("syntax_enum.rs");
    std::fs::write(&out_file, out_stream.as_bytes()).expect("Could not write syntax_enum.rs");

    p!("syntax: {:?}", out_file);
}
