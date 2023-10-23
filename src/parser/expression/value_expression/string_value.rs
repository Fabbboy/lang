use crate::parser::expression::Expression;

use super::ValueExpression;

#[derive(Debug)]
pub struct StringValue {
  pub value: String,
}

impl StringValue {
  pub fn new(value: String) -> Self {
    StringValue { value }
  }

  fn expression(&self, value: String) -> Expression {
    Expression::Value(ValueExpression::String(StringValue::new(value)))
  }
}
