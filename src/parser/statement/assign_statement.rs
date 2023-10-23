use crate::parser::expression::Expression;

use super::Statement;

#[derive(Debug)]
pub struct AssignStatement {
  pub variable_expr: Expression,
  pub value_expr: Expression,
  pub is_mutable: bool,
}

impl AssignStatement {
  pub fn new(variable_expr: Expression, value_expr: Expression, is_mutable: bool) -> Self {
    AssignStatement {
      variable_expr,
      value_expr,
      is_mutable,
    }
  }

  fn statement(
    &self,
    variable_expr: Expression,
    value_expr: Expression,
    is_mutable: bool,
  ) -> Statement {
    Statement::Assign(AssignStatement::new(variable_expr, value_expr, is_mutable))
  }
}
