use super::Expression;

#[derive(Debug)]
pub struct VariableExpression {
  pub name: String,
  pub type_: Box<Expression>,
}

impl VariableExpression {
  pub fn new(name: String, type_: Box<Expression>) -> Self {
    VariableExpression { name, type_ }
  }
}
