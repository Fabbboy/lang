#[derive(Debug, PartialEq)]
pub struct IntValue {
  pub value: i32,
}

impl IntValue {
  pub fn new(value: i32) -> Self {
    IntValue { value }
  }
}
