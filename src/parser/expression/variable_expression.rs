use super::Expression;

#[derive(Debug)]
pub struct VariableExpression {
  pub name: String,
  pub type_: Box<Expression>,
}
