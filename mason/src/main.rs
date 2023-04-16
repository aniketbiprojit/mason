fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("No filename provided");

    let source = std::fs::read_to_string(filename).expect("Could not read file");

    let lexer = &mut lexer::Lexer::new(&source);
    lexer.tokenize();

    for token in &lexer.tokens {
        println!("{:?}", token);
    }
}
