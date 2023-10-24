#[derive(Debug, PartialEq)]
pub enum Type {
  Byte,
  Int,
  Float,
  String,
  Void,
}
#[derive(Debug, PartialEq)]
pub struct TypeExpression {
  pub type_: Type,
}

impl TypeExpression {
  pub fn new(type_: Type) -> Self {
    TypeExpression { type_ }
  }
}
