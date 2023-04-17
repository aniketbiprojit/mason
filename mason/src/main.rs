use std::path::PathBuf;

use parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("No filename provided");

    let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = binding.ancestors().nth(1).unwrap();

    println!("root: {:?}", root);
    let file_path = root.join(filename);

    let mut source = std::fs::read_to_string(&file_path).expect("Could not read file");

    source += "\n";

    let lexer = &mut lexer::Lexer::new(&source);
    lexer.tokenize();

    #[cfg(feature = "debug")]
    {
        let serialized = lexer.serial();

        std::fs::write(".debug/debug-lexer-output.json", serialized)
            .expect("Could not write to file");
    }
    let parser = Parser::new(&lexer.tokens);

    parser.parse();
}
