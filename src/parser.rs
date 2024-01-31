use crate::lexer;
use crate::error;

type ParseResult<'t> = Result<f64, error::Error<'t>>;

fn parse_value<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  match tokens.peek().ttype {
    lexer::TokenType::Number(value) => {
      tokens.next();
      Ok(value)
    }
    _ => Err(error::Error::new(
      format!("Expected number or '(', found {:?}", tokens.peek()),
      tokens.tokens,
      tokens.peek().span,
    )),
  }
}

fn parse_parens<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  match tokens.peek().ttype {
    lexer::TokenType::LParen => {
      tokens.next();
      let result = parse_sum(tokens)?;
      match tokens.peek().ttype {
        lexer::TokenType::RParen => {
          tokens.next();
          Ok(result)
        }
        _ => Err(error::Error::new(
          format!("Expected ')', found '{:?}'", tokens.peek()),
          tokens.tokens,
          tokens.peek().span,
        )),
      }
    }
    _ => parse_value(tokens),
  }
}

fn parse_product<'t>(tokens: &mut lexer::TokensIter<'t>) -> ParseResult<'t> {
  let mut result = parse_parens(tokens)?;
  loop {
    match tokens.peek().ttype {
      lexer::TokenType::Mul => {
        tokens.next();
        result *= &parse_parens(tokens)?;
      }
      lexer::TokenType::Div => {
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
    match tokens.peek().ttype {
      lexer::TokenType::Add => {
        tokens.next();
        result += &parse_product(tokens)?;
      }
      lexer::TokenType::Sub => {
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

  match iter.peek().ttype {
    lexer::TokenType::EOF => Ok(result),
    _ => Err(error::Error::new(
      format!(
        "Expected end of input, '+', '-', '*', or '/', found {:?}",
        iter.peek()
      ),
      tokens,
      iter.peek().span,
    )),
  }
}
