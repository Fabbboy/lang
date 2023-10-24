#[derive(Debug, PartialEq)]
pub struct StringValue {
  pub value: String,
}

impl StringValue {
  pub fn new(value: String) -> Self {
    StringValue { value }
  }
}
