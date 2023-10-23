use crate::parser::expression::Expression;

use super::ValueExpression;

#[derive(Debug)]
pub struct MemoryValue {
  pub value: String,
}

impl MemoryValue {
  fn new(value: String) -> Self {
    MemoryValue { value }
  }

  fn expression(&self, value: String) -> Expression {
    Expression::Value(ValueExpression::Memory(MemoryValue::new(value)))
  }
}
