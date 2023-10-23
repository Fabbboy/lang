use crate::parser::{expression::value_expression::ValueExpression, sizes::Size};

use super::Statement;

#[derive(Debug)]
pub struct MovStatement {
  size: Size,
  destination: ValueExpression,
  source: ValueExpression,
}

impl MovStatement {
  pub fn new(size: Size, destination: ValueExpression, source: ValueExpression) -> Self {
    MovStatement {
      size,
      destination,
      source,
    }
  }

  pub fn statement(
    &self,
    size: Size,
    destination: ValueExpression,
    source: ValueExpression,
  ) -> Statement {
    Statement::Mov(MovStatement::new(size, destination, source))
  }
}
