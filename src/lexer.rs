use lazy_static::lazy_static;
use regex::Regex;

pub fn tokenize(input: &str) -> Vec<Token> {
  // Regex pattern for each token
  lazy_static! {
    static ref PATTERN: Regex = Regex::new(
      "\\s*([+\\-*/()]|\\d+(?:\\.\\d+)?)"  // TODO: craft a regex that matches all tokens
    ).unwrap();
  };

  let mut prev_offset = 0;

  PATTERN.captures_iter(input).map_while(|capt| {
    let token = capt.get(1)?;  // actual token text (no whitespace)
    let whole = capt.get(0)?;  // whole match (including whitespace)
    if whole.start() > prev_offset {  // check for unmatched characters
      None
    } else {
      prev_offset = whole.end();  // update offset
      Some(Token::from(&token))
    }
  }).collect()
}

pub struct Token<'t> {
  pub text: &'t str,
  pub start: usize,
  pub end: usize,
}

impl<'t> Token<'t> {
  pub fn from(m: &regex::Match<'t>) -> Self {
    Self {
      start: m.start(),
      end: m.end(),
      text: m.as_str(),
    }
  }
}
