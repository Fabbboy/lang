use crate::parser::expression::Expression;

use super::ValueExpression;


#[derive(Debug)]
pub enum Register {
  R01,
  R02,
  R03,
  R04,
  R05,
  Rsp,
  Rip,
  Rbp,
  R06,
}


#[derive(Debug)]
pub enum RegisterValue {
  Register(Register),
  RegisterOffset(Register, i32),
}

impl RegisterValue {
  pub fn new(register: Register) -> Self {
    RegisterValue::Register(register)
  }

  pub fn new_offset(register: Register, offset: i32) -> Self {
    RegisterValue::RegisterOffset(register, offset)
  }

  pub fn expression(&self, register: Register) -> Expression {
    Expression::Value(ValueExpression::Register(RegisterValue::new(register)))
  }

  pub fn expression_offset(&self, register: Register, offset: i32) -> Expression {
    Expression::Value(ValueExpression::Register(RegisterValue::new_offset(
      register, offset,
    )))
  }
}
