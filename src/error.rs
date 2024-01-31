use crate::lexer;

pub struct Error<'t> {
  pub message: String,
  pub tokens: &'t lexer::Tokens<'t>,
  pub span: lexer::Span
}

impl<'t> Error<'t> {
  pub fn new(
    message: String, tokens: &'t lexer::Tokens<'t>, span: lexer::Span
  ) -> Self {
    Self { message, tokens, span }
  }
}

impl<'t> std::fmt::Debug for Error<'t> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "\n\n  \x1B[31mParsing error:\x1B[m\n")?;
    self.span.print(f, self.tokens)?;
    writeln!(f, "     :  \x1B[31m{}\x1B[m", self.message)
  }
}
