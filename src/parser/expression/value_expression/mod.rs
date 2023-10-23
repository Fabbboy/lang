pub mod FloatValue;
pub mod IntValue;
pub mod MemoryValue;
pub mod RegisterValue;
pub mod StringValue;

#[derive(Debug)]
pub enum ValueExpression {
  Int(IntValue::IntValue),
  Float(FloatValue::FloatValue),
  String(StringValue::StringValue),
  Register(RegisterValue::RegisterValue),
  Memory(MemoryValue::MemoryValue),
}
