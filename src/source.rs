use std::vec;

pub struct Source {
  pub text: String,
  pub lines: Vec<usize>,
}

impl Source {
  pub fn new(text: String) -> Source {
    let lines = find_lines(&text);
    Source { text, lines }
  }

  // Returns line number/index, for the given offset
  pub fn get_line(&self, offset: usize) -> usize {
    match self.lines.binary_search(&offset) {
      Ok(idx) => idx,
      Err(idx) => idx - 1,
    }
  }

  // Returns a slice of the line
  pub fn get_line_slice(&self, line: usize) -> &str {
    let start = self.lines[line];
    let &end = self.lines.get(line + 1).unwrap_or(&self.text.len());
    &self.text[start..end]
  }
}

// Returns a vector of indices of line beginnings
fn find_lines(text: &str) -> Vec<usize> {
  let mut lines = vec![0];
  for (i, c) in text.chars().enumerate() {
    if c == '\n' {
      lines.push(i + 1);
    }
  }
  lines
}
