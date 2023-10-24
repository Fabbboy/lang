use crate::lexer::{codepos::CodePos, TokenT};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token: TokenT,
  pub pos: CodePos,
  pub value: Option<String>,
}

impl Token {
  pub fn new(token: TokenT, pos: CodePos, value: Option<String>) -> Token {
    Token {
      token: token,
      pos: pos,
      value: value,
    }
  }
  pub fn zero() -> Token {
    Token {
      token: TokenT::WHITESPACE,
      pos: CodePos::zero(),
      value: Option::from(String::new()),
    }
  }
}
