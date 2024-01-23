mod error;
mod lexer;
mod parser;
mod source;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("\n\nParser:\n\n");

  // Read command line arguments
  let file_name = args.get(1)
    .expect("Error: expected source file name as first argument");
  
  // Read source code
  let code = std::fs::read_to_string(file_name)
    .expect("Error: could not read source file");

  // Construct source object
  let source = source::Source::new(code);

  // Tokenize
  let tokens = lexer::tokenize(&source);
  
  // Parse
  let parsed = parser::parse(&tokens).unwrap();

  // Print result
  println!("Parsed expression successfully: {parsed:.2}\n\n");
}

