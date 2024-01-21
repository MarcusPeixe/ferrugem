use crate::lexer;

fn peek<'t, T>(tokens: &mut std::iter::Peekable<T>) -> Option<&'t str>
where
  T: Iterator<Item = &'t lexer::Token<'t>>
{
  tokens.peek().and_then(|t| Some(t.text))
}

fn parse_value<'t, T>(tokens: &mut std::iter::Peekable<T>) -> Result<f64, String>
where
  T: Iterator<Item = &'t lexer::Token<'t>>
{
  match peek(tokens) {
    Some(text) => {
      tokens.next();
      match text.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("Expected number, found '{}'", text)),
      }
    }
    None => Err(format!("Expected number, found end of input")),
  }
}

fn parse_parens<'t, T>(tokens: &mut std::iter::Peekable<T>) -> Result<f64, String>
where
  T: Iterator<Item = &'t lexer::Token<'t>>
{
  match peek(tokens) {
    Some("(") => {
      tokens.next();
      let result = parse_sum(tokens)?;
      match peek(tokens) {
        Some(")") => {
          tokens.next();
          Ok(result)
        }
        Some(text) => Err(format!("Expected ')', found '{}'", text)),
        None => Err(format!("Expected ')', found end of input")),
      }
    }
    Some(_) => parse_value(tokens),
    None => Err(format!("Expected number or '(', found end of input")),
  }
}

fn parse_product<'t, T>(tokens: &mut std::iter::Peekable<T>) -> Result<f64, String>
where
  T: Iterator<Item = &'t lexer::Token<'t>>
{
  let mut result = parse_parens(tokens)?;
  loop {
    match peek(tokens) {
      Some("*") => {
        tokens.next();
        result *= &parse_parens(tokens)?;
      }
      Some("/") => {
        tokens.next();
        result /= &parse_parens(tokens)?;
      }
      _ => break,
    }
  }
  Ok(result)
}

fn parse_sum<'t, T>(tokens: &mut std::iter::Peekable<T>) -> Result<f64, String>
where
  T: Iterator<Item = &'t lexer::Token<'t>>
{
  let mut result = parse_product(tokens)?;
  loop {
    match peek(tokens) {
      Some("+") => {
        tokens.next();
        result += &parse_product(tokens)?;
      }
      Some("-") => {
        tokens.next();
        result -= &parse_product(tokens)?;
      }
      _ => break,
    }
  }
  Ok(result)
}

pub fn parse(tokens: &Vec<lexer::Token>) -> Result<f64, String> {
  let mut iter = tokens.iter().peekable();
  
  // Parse here!
  let result = parse_sum(&mut iter)?;

  if iter.peek().is_none() {
    Ok(result)
  }
  else {
    // TODO: Create a proper error type
    Err(format!("Unexpected trailing token '{}'", iter.peek().unwrap().text))
  }
}
