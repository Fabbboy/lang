#[derive(Debug)]
pub struct FloatValue {
  pub value: f32,
}

impl FloatValue {
  pub fn new(value: f32) -> Self {
    FloatValue { value }
  }
}
