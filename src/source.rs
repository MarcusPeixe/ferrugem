pub struct Source {
  pub text: String,
  pub lines: Vec<usize>,
}

impl Source {
  pub fn new(text: String) -> Source {
    let lines = find_lines(&text);
    Source { text, lines }
  }
}

// Returns a vector of indices of line beginnings
fn find_lines(text: &str) -> Vec<usize> {
  text.char_indices()
    .filter(|(_, c)| *c == '\n')
    .map(|(i, _)| i + 1)
    .collect()
}
