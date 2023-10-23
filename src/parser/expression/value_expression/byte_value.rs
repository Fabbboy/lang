#[derive(Debug)]
pub struct ByteValue {
  pub value: i8,
}

impl ByteValue {
  pub fn new(value: i8) -> Self {
    ByteValue { value }
  }
}
