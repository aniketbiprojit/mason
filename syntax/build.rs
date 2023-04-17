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

#[derive(Debug, Deserialize, Clone)]
struct Kind {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct SyntaxKind {
    directive: Vec<Kind>,
    #[serde(rename = "identifier")]
    outer_identifier: Vec<String>,
    #[serde(skip)]
    identifier: Vec<Kind>,
    literal: Vec<Kind>,
    double_punctuation: Vec<Kind>,
    punctuation: Vec<Kind>,
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

    let mut syntax =
        serde_json::from_str::<SyntaxKind>(&syntax).expect("Could not parse syntax file");

    syntax.identifier = syntax
        .outer_identifier
        .iter()
        .map(|name| Kind {
            name: name.to_string(),
            value: name.to_string(),
        })
        .collect();

    let mut out_stream = String::new();

    out_stream.push_str(&format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]",
        "pub enum TokenType {",
        "   Punctuation,",
        "   Literal,",
        "   Identifier,",
        "}\n\n"
    ));

    out_stream.push_str(&format!(
        "{}\n{}",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]", "pub enum SyntaxKind {"
    ));

    let vec: Vec<&Kind> = syntax
        .double_punctuation
        .iter()
        .chain(syntax.punctuation.iter())
        .chain(syntax.literal.iter())
        .chain(syntax.identifier.iter())
        .chain(syntax.directive.iter())
        .collect();

    for token in vec {
        out_stream.push_str(&format!("\n    {},", token.name.to_case(Case::UpperCamel)));
    }

    out_stream.push_str("\n}");

    out_stream.push_str(
        "\n\n#[derive(Debug, Clone)]\npub struct TokenMetadata {\
        \n    pub kind: SyntaxKind,\
        \n    pub token_type: TokenType,\n\
        \n    pub text: String,\n\
    }",
    );

    out_stream.push_str("\n\npub fn match_operator(token: &str) -> Option<TokenMetadata> {");
    out_stream.push_str("\n    let token = match token {");

    let vec: Vec<&Kind> = syntax
        .double_punctuation
        .iter()
        .chain(syntax.punctuation.iter())
        .collect();

    for token in &vec {
        out_stream.push_str(&format!(
            "\n        x if x.starts_with(\"{}\") => Some(TokenMetadata {{
            kind: SyntaxKind::{}, 
            token_type: TokenType::Punctuation,
            text: \"{}\".to_string(),
        }}),",
            token.value,
            token.name.to_case(Case::UpperCamel),
            token.value,
        ));
    }

    out_stream.push_str(&format!("\n        {} => {},", "_", "None"));

    out_stream.push_str("\n    };");
    out_stream.push_str("\n    token");
    out_stream.push_str("\n}");

    out_stream.push_str("\n\npub fn match_identifier(token: &str) -> Option<TokenMetadata> {");
    out_stream.push_str("\n    let token = match token {");

    let vec: Vec<&Kind> = syntax
        .identifier
        .iter()
        .chain(syntax.directive.iter())
        .collect();

    for token in vec {
        out_stream.push_str(&format!(
            "\n        \"{}\" => Some(TokenMetadata {{
            kind: SyntaxKind::{}, 
            token_type: TokenType::Identifier,
            text: \"{}\".to_string(),
        }}),",
            token.value,
            token.name.to_case(Case::UpperCamel),
            token.value
        ));
    }

    out_stream.push_str(&format!("\n        {} => {},", "_", "None"));

    out_stream.push_str("\n    };");
    out_stream.push_str("\n    token");
    out_stream.push_str("\n}");

    let out_dir =
        std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not get OUT_DIR"));

    let out_file = out_dir.join("syntax_enum.rs");
    std::fs::write(&out_file, out_stream.as_bytes()).expect("Could not write syntax_enum.rs");

    p!("syntax: {:?}", out_file);
}
