use lazy_static::lazy_static;
use regex::Regex;
use crate::{error, source};

// Regex pattern for each token
lazy_static! {
  static ref PATTERN: Regex = Regex::new(
    // TODO: craft a regex that matches all tokens
    "^\\s*([+\\-*/()]|\\d+(?:\\.\\d+)?)"
  ).unwrap();
}

fn next_token(input: &str) -> Option<(regex::Match, usize)> {
  let regex_match = PATTERN.captures(input)?;
  let whole = regex_match.get(0)?;
  let token = regex_match.get(1)?;

  Some((token, whole.end()))
}

type TokenizeResult<'t> = Result<Tokens<'t>, error::Error<'t>>;

pub fn tokenize<'t>(input: &'t source::Source) -> TokenizeResult<'t> {
  let text = input.text.as_str();

  let mut position: usize = 0;
  let mut tokens = Vec::new();

  while let Some((token, length)) = next_token(&text[position..]) {
    tokens.push(Token::from(&token, &input, position));
    position += length;
  }

  if text[position..].trim().is_empty() {
    Ok(Tokens { source: input, tokens })
  } else {
    Err(error::Error::new_span(
      format!("Unexpected characters"),
      Span {
        source: &input,
        start: position,
        end: input.text.len(),
      },
    ))
  }
}

pub struct Tokens<'t> {
  pub source: &'t source::Source,
  pub tokens: Vec<Token<'t>>,
}

impl<'t> Tokens<'t> {
  pub fn iter(&'t self) -> TokensIter<'t> {
    TokensIter::new(self)
  }
}

#[derive(Clone, Copy)]
pub struct TokensIter<'t> {
  tokens: &'t Tokens<'t>,
  index: usize,
}

type IterItem<'t> = (&'t str, &'t Span<'t>);

impl<'t> TokensIter<'t> {
  pub fn new(tokens: &'t Tokens<'t>) -> Self {
    Self {
      tokens,
      index: 0,
    }
  }
  
  pub fn peek(&mut self) -> Option<IterItem<'t>> {
    Some(self.tokens.tokens.get(self.index)?.tuple())
  }
}

impl<'t> Iterator for TokensIter<'t> {
  type Item = IterItem<'t>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.tokens.tokens.get(self.index)?.tuple();
    self.index += 1;
    Some(result)
  }
}

#[derive(Clone, Copy)]
pub struct Span<'t> {
  pub source: &'t source::Source,
  pub start: usize,
  pub end: usize,
}

impl<'t> Span<'t> {
  // TODO: Improve span printing
  pub fn print(&self) {
    eprintln!(
      "{}\x1B[31m{}\x1B[39m{}",
      &self.source.text[..self.start],
      &self.source.text[self.start..self.end],
      &self.source.text[self.end..],
    );
  }
}

pub struct Token<'t> {
  pub span: Span<'t>,
}

impl<'t> Token<'t> {
  pub fn from(m: &regex::Match<'t>, src: &'t source::Source, pos: usize) -> Self {
    Self {
      span: Span {
        source: src,
        start: pos + m.start(),
        end: pos + m.end(),
      },
    }
  }

  pub fn get_text(&self) -> &str {
    &self.span.source.text[self.span.start..self.span.end]
  }

  pub fn tuple(&self) -> (&str, &Span<'t>) {
    (self.get_text(), &self.span)
  }
}
