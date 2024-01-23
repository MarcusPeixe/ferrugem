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

pub fn tokenize<'t>(input: &'t source::Source) -> Tokens<'t> {
  let text = input.text.as_str();

  let mut position: usize = 0;
  let mut tokens = Vec::new();

  while let Some((token, length)) = next_token(&text[position..]) {
    tokens.push(Token::from(&token, position));
    position += length;
  }

  let tokens = Tokens { source: input, tokens };
  if !text[position..].trim().is_empty() {
    let error = error::Error::new(
      format!("Unrecognized trailing characters"),
      &tokens,
      Span {
        start: position,
        end: input.text.len(),
      },
    );
    panic!("{error:?}");
  }
  tokens
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
  pub tokens: &'t Tokens<'t>,
  pub index: usize,
}

type IterItem<'t> = (&'t str, &'t Span);

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
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl Span {
  // TODO: Improve span printing
  pub fn print<'t>(
    &self, f: &mut std::fmt::Formatter<'_>, tokens: &'t Tokens<'t>
  ) -> std::fmt::Result
  {
    let line_start = tokens.source.get_line(self.start);
    let line_end = tokens.source.get_line(self.end);
    let offset_start = self.start -  tokens.source.lines[line_start];
    let offset_end = self.end - tokens.source.lines[line_end];

    for i in line_start..=line_end {
      let line = tokens.source.get_line_slice(i).trim_end();
      let start = if i == line_start { offset_start } else { 0 };
      let end = if i == line_end { offset_end } else { line.len() };

      writeln!(
        f, " {:3} |  {}\x1B[1;31m{}\x1B[m{}",
        i + 1, &line[..start], &line[start..end], &line[end..]
      )?;
      writeln!(
        f, "     |  {}\x1B[1;31m{}\x1B[m",
        " ".repeat(start),
        "^".repeat(std::cmp::max(end - start, 1)),
      )?;
    }

    Ok(())
  }
}

pub struct Token<'t> {
  pub text: &'t str,
  pub span: Span,
}

impl<'t> Token<'t> {
  pub fn from(m: &regex::Match<'t>, pos: usize) -> Self {
    Self {
      text: m.as_str(),
      span: Span {
        start: pos + m.start(),
        end: pos + m.end(),
      },
    }
  }

  pub fn tuple(&self) -> (&str, &Span) {
    (self.text, &self.span)
  }
}
