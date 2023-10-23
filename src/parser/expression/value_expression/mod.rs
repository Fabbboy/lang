pub mod byte_value;
pub mod float_value;
pub mod int_value;
pub mod string_value;

#[derive(Debug)]
pub enum ValueExpression {
  Byte(byte_value::ByteValue),
  Int(int_value::IntValue),
  Float(float_value::FloatValue),
  String(string_value::StringValue),
  Void,
}
