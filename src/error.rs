use crate::lexer;

pub struct Error<'t> {
  pub message: String,
  pub span: Option<lexer::Span<'t>>
}

impl<'t> Error<'t> {
  pub fn new(message: String) -> Self {
    Self { message, span: None }
  }

  pub fn new_span(message: String, span: lexer::Span<'t>) -> Self {
    Self { message, span: Some(span) }
  }

  // TODO: implement correct traits for this
  pub fn to_string(&self) -> String {
    if let Some(span) = self.span {
      span.print();
    }
    self.message.clone()
  }
}
