use super::Expression;

#[derive(Debug, PartialEq)]
pub struct VariableExpression {
  pub name: String,
  pub type_: Option<Box<Expression>>,
}

impl VariableExpression {
  pub fn new(name: String, type_: Option<Box<Expression>>) -> Self {
    VariableExpression { name, type_ }
  }
}
