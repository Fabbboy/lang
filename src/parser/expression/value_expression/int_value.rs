use crate::parser::expression::Expression;

use super::ValueExpression;

#[derive(Debug)]
pub struct IntValue {
  pub value: i32,
}

impl IntValue {
  fn new(value: i32) -> Self {
    IntValue { value }
  }

  fn expression(&self, value: i32) -> Expression {
    Expression::Value(ValueExpression::Int(IntValue::new(value)))
  }
}
