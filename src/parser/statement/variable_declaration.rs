use crate::parser::expression::Expression;

#[derive(Debug)]
pub struct VariableDeclaration {
  pub variable_expr: Expression,
  pub value_expr: Expression,
  pub is_const: bool,
}
