use crate::parser::expression::Expression;

use super::Statement;

#[derive(Debug)]
pub struct AssignStatement {
  pub variable_expr: Expression,
  pub value_expr: Expression,
  pub is_const: bool,
}

impl AssignStatement {
  fn new(variable_expr: Expression, value_expr: Expression, is_const: bool) -> Self {
    AssignStatement {
      variable_expr,
      value_expr,
      is_const,
    }
  }

  fn statement(
    &self,
    variable_expr: Expression,
    value_expr: Expression,
    is_const: bool,
  ) -> Statement {
    Statement::Assign(AssignStatement::new(variable_expr, value_expr, is_const))
  }
}
