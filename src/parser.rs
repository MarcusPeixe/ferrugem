use crate::lexer;
use crate::error;

type ParseResult<'t> = Result<f64, error::Error<'t>>;

fn parse_value<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  match tokens.peek() {
    Some((text, &span)) => {
      let value = match text.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(error::Error::new(
          format!("Expected number, found '{}'", text),
          tokens.tokens,
          span,
        )),
      };
      tokens.next();
      value
    }
    None => Err(error::Error::new_end(
      format!("Expected number, found end of input"),
      tokens.tokens,
    )),
  }
}

fn parse_parens<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  match tokens.peek() {
    Some(("(", _)) => {
      tokens.next();
      let result = parse_sum(tokens)?;
      match tokens.peek() {
        Some((")", _)) => {
          tokens.next();
          Ok(result)
        }
        Some((text, &span)) => Err(error::Error::new(
          format!("Expected ')', found '{}'", text),
          tokens.tokens,
          span,
        )),
        None => Err(error::Error::new_end(
          format!("Expected ')', found end of input"),
          tokens.tokens,
        )),
      }
    }
    Some(_) => parse_value(tokens),
    None => Err(error::Error::new_end(
      format!("Expected number or '(', found end of input"),
      tokens.tokens,
    )),
  }
}

fn parse_product<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  let mut result = parse_parens(tokens)?;
  loop {
    match tokens.peek() {
      Some(("*", _)) => {
        tokens.next();
        result *= &parse_parens(tokens)?;
      }
      Some(("/", _)) => {
        tokens.next();
        result /= &parse_parens(tokens)?;
      }
      _ => break,
    }
  }
  Ok(result)
}

fn parse_sum<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  let mut result = parse_product(tokens)?;
  loop {
    match tokens.peek() {
      Some(("+", _)) => {
        tokens.next();
        result += &parse_product(tokens)?;
      }
      Some(("-", _)) => {
        tokens.next();
        result -= &parse_product(tokens)?;
      }
      _ => break,
    }
  }
  Ok(result)
}

pub fn parse<'t>(tokens: &'t lexer::Tokens) -> ParseResult<'t> {
  let mut iter = tokens.iter();
  
  // Parse here!
  let result = parse_sum(&mut iter)?;

  if iter.peek().is_none() {
    Ok(result)
  }
  else {
    let (text, &span) = iter.peek().unwrap();
    // TODO: Create a proper error type
    Err(error::Error::new(
      format!("Unexpected trailing token '{}'", text),
      tokens,
      span,
    ))
  }
}
