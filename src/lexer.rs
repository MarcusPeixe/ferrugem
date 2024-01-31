use lazy_static::lazy_static;
use regex::Regex;
use crate::{error, source};

// Regex pattern for each token
lazy_static! {
  static ref PATTERN: Regex = Regex::new(
    // TODO: craft a regex that matches all tokens
    "^\\s*([+\\-*/()]|\\d+(?:\\.\\d+)?)\\s*"
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
  tokens.push(Token::eof(position));

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
    Err::<(), error::Error>(error).unwrap();
  }
  tokens
}

////////////////////////////////////////////////////////////////////////////////

pub struct Tokens<'t> {
  pub source: &'t source::Source,
  pub tokens: Vec<Token<'t>>,
}

impl<'t> Tokens<'t> {
  pub fn iter(&'t self) -> TokensIter<'t> {
    TokensIter::new(self)
  }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct TokensIter<'t> {
  pub tokens: &'t Tokens<'t>,
  pub index: usize,
}

impl<'t> TokensIter<'t> {
  pub fn new(tokens: &'t Tokens<'t>) -> Self {
    Self {
      tokens,
      index: 0,
    }
  }
  
  pub fn peek(&mut self) -> &Token<'t> {
    &self.tokens.tokens[self.index]
  }
}

impl<'t> Iterator for TokensIter<'t> {
  type Item = &'t Token<'t>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.tokens.tokens.get(self.index)?;
    self.index += 1;
    Some(result)
  }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl Span {
  pub fn print<'t>(
    &self, f: &mut std::fmt::Formatter<'_>, tokens: &'t Tokens<'t>
  ) -> std::fmt::Result
  {
    let line_start = tokens.source.get_line(self.start);
    let line_end = tokens.source.get_line(self.end);
    let offset_start = self.start - tokens.source.lines[line_start];
    let offset_end = self.end - tokens.source.lines[line_end];

    for i in line_start..=line_end {
      let line = tokens.source.get_line_slice(i).trim_end();
      let start = if i == line_start { offset_start } else { 0 };
      let end = if i == line_end { offset_end } else { line.len() };

      print_line(f, tokens, i, self.start, self.end)?;
      writeln!(
        f, "     |  {}\x1B[1;31m{}\x1B[m",
        " ".repeat(start),
        "^".repeat(std::cmp::max(end - start, 1)),
      )?;
    }

    Ok(())
  }
}

fn print_line(
  f: &mut std::fmt::Formatter<'_>, tokens: &Tokens<'_>,
  line: usize, start: usize, end: usize,
) -> std::fmt::Result
{
  let start_offset = tokens.source.lines[line];
  let end_offset = *tokens.source.lines.get(line + 1)
    .unwrap_or(&tokens.source.text.len());

  write!(f, " {:3} |  ", line + 1)?;

  let mut curr_token = tokens.tokens.binary_search_by(|t| {
    if t.span.end <= start_offset {
      std::cmp::Ordering::Less
    } else if t.span.start >= start_offset {
      std::cmp::Ordering::Greater
    } else {
      std::cmp::Ordering::Equal
    }
  }).unwrap_or_else(|p| p);

  let mut curr_color = "";
  let slice = &tokens.source.text[start_offset..end_offset].trim_end();
  for (i, c) in slice.chars().enumerate() {
    let i = i + start_offset;
    let mut new_color = "";
    if curr_token < tokens.tokens.len() {
      let span_start = tokens.tokens[curr_token].span.start;
      let span_end = tokens.tokens[curr_token].span.end;
      if i >= span_start && i < span_end {
        new_color = match tokens.tokens[curr_token].ttype {
          TokenType::Number(_) => ";32",
          TokenType::LParen | TokenType::RParen
          | TokenType::Add | TokenType::Sub
          | TokenType::Mul | TokenType::Div => ";1;35",
          TokenType::EOF => "",
        };
      }
      if i == tokens.tokens[curr_token].span.end - 1 {
        curr_token += 1;
      }
    }
    if i >= start && i < end {
      new_color = ";1;31";
    }
    if new_color != curr_color {
      write!(f, "\x1B[{}m", new_color)?;
      curr_color = new_color;
    }
    write!(f, "{}", c)?;
  }

  writeln!(f, "\x1B[m")
}

////////////////////////////////////////////////////////////////////////////////

pub enum TokenType {
  Number(f64),
  LParen, RParen,
  Add, Sub, Mul, Div,
  EOF,
}

pub struct Token<'t> {
  pub text: &'t str,
  pub ttype: TokenType,
  pub span: Span,
}

impl<'t> Token<'t> {
  pub fn from(m: &regex::Match<'t>, pos: usize) -> Self {
    Self {
      text: m.as_str(),
      ttype: determine_type(m.as_str()),
      span: Span {
        start: pos + m.start(),
        end: pos + m.end(),
      },
    }
  }

  pub fn eof(pos: usize) -> Self {
    Self {
      text: "",
      ttype: TokenType::EOF,
      span: Span { start: pos, end: pos },
    }
  }
}

impl<'t> std::fmt::Debug for Token<'t> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self.ttype {
      TokenType::EOF => write!(f, "end of input"),
      _ => write!(f, "'{}'", self.text),
    }
  }
}

fn determine_type(text: &str) -> TokenType {
  match text {
    "(" => TokenType::LParen,
    ")" => TokenType::RParen,
    "+" => TokenType::Add,
    "-" => TokenType::Sub,
    "*" => TokenType::Mul,
    "/" => TokenType::Div,
    _   => TokenType::Number(text.parse().unwrap()),
  }
}
