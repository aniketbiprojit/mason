pub mod lexer;

fn main() {
    let file_path = &"hello.c";
    let contents = std::fs::read_to_string(file_path).expect("Failed to read file");

    let lexer = lexer::Lexer::new(&contents);

    println!("The file contents are: \n\n{:}", lexer.source);
}
