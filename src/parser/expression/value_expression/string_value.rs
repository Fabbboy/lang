#[derive(Debug)]
pub struct StringValue {
  pub value: String,
}

impl StringValue {
  pub fn new(value: String) -> Self {
    StringValue { value }
  }
}
