use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("No filename provided");

    let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = binding.ancestors().nth(1).unwrap();

    println!("root: {:?}", root);
    let file_path = root.join(filename);

    let source = std::fs::read_to_string(&file_path).expect("Could not read file");

    let lexer = &mut lexer::Lexer::new(&source);
    lexer.tokenize();
    #[cfg(feature = "debug")]
    {
        for directive in &lexer.directives {
            println!("directive: {:?}", directive);
        }

        for token in &lexer.tokens {
            println!("{:?}", token);
        }
    }
}
