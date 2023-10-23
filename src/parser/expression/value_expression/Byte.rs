use crate::parser::expression::Expression;

use super::ValueExpression;

#[derive(Debug)]
pub struct ByteValue {
  pub value: i8,
}

impl ByteValue {
  pub fn new(value: i8) -> Self {
    ByteValue { value }
  }

  fn expression(&self, value: i8) -> Expression {
    Expression::Value(ValueExpression::Byte(ByteValue::new(value)))
  }
}
