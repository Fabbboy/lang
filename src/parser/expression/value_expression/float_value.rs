use crate::parser::expression::Expression;

use super::ValueExpression;

#[derive(Debug)]
pub struct FloatValue {
  pub value: f32,
}

impl FloatValue {
  pub fn new(value: f32) -> Self {
    FloatValue { value }
  }

  fn expression(&self, value: f32) -> Expression {
    Expression::Value(ValueExpression::Float(FloatValue::new(value)))
  }
}
