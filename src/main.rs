mod tokenizer;
mod parser;

use std::fs;

fn main() {
    // 1. Read input
    let input = fs::read_to_string("Main.jack").expect("Main.jack missing");

    // 2. Tokenize and generate MainT.xml
    let tokenizer = tokenizer::JackTokenizer::new(input);
    fs::write("MainT.xml", tokenizer.to_xml()).expect("Failed to write MainT.xml");
    println!("Generated MainT.xml (Tokenizer output)");

    // 3. Parse and generate Main.xml
    let mut parser = parser::Parser::new(tokenizer);
    let parse_tree = parser.compile_class();
    fs::write("Main.xml", parse_tree).expect("Failed to write Main.xml");
    println!("Generated Main.xml (Parser output)");
}