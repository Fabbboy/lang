use super::Expression;
#[derive(Debug)]
pub enum Type {
  Byte,
  Int,
  Float,
  String,
  Void
}
#[derive(Debug)]
pub struct TypeExpression {
  pub type_: Type,
}

impl TypeExpression {
  pub fn new(type_: Type) -> Self {
    TypeExpression { type_ }
  }

  pub fn expression(&self, type_: Type) -> Expression {
    Expression::Type(TypeExpression::new(type_))
  }
}
