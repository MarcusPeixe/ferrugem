mod lexer;
mod parser;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let name = args.get(1)
    .expect("Error: expected source file name as first argument");
  
  let source = std::fs::read_to_string(name)
    .expect(&format!("Error: failed to read source file '{}'", name));

  println!("\n\nInput:\n===\n{}\n===", source);

  // Tokenize
  let tokens = lexer::tokenize(&source);
  print!("Tokens: ");
  for token in &tokens {
    print!("'{}' ", token.text);
  }
  println!();

  // Parse
  match parser::parse(&tokens) {
    Ok(parsed) => {
      println!("Parsed expression: {:.2}", parsed);
    }
    Err(e) => {
      println!("Failed to parse expression: {}", e);
    }
  }
}

