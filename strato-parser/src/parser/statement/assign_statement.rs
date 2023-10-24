use crate::parser::expression::Expression;

#[derive(Debug, PartialEq)]
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
}
