mod error;
mod lexer;
mod parser;
mod source;

fn main() -> Result<(), String> {
  let args: Vec<String> = std::env::args().collect();

  // let (term_width, term_height) = termion::terminal_size().unwrap();
  // println!("Size: {}x{}", term_width, term_height);
  println!("\n");

  // Read command line arguments
  let name = args.get(1)
    .expect("Error: expected source file name as first argument");
  
  // Read source code
  let code = match std::fs::read_to_string(name) {
    Ok(code) => code,
    Err(error) => return Err(error.to_string()),
  };

  // Construct source object
  let source = source::Source::new(code);

  // Tokenize
  let tokens = match lexer::tokenize(&source) {
    Ok(tokens) => tokens,
    Err(error) => return error.return_in_main(),
  };

  // Parse
  let parsed = match parser::parse(&tokens) {
    Ok(tokens) => tokens,
    Err(error) => return error.return_in_main(),
  };

  // Print result
  println!("Parsed expression: {:.2}", parsed);

  Ok(())
}

