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

  // pub fn print(&self) {
  //   if let Some(span) = self.span {
  //     span.print();
  //   }
  //   eprintln!("Error: {}", self.message);
  // }

  pub fn return_in_main(&self) -> Result<(), String> {
    if let Some(span) = self.span {
      span.print();
    }
    Err(self.message.clone())
  }
}
