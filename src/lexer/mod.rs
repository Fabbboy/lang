use regex::Regex;

use self::token::Token;

pub mod codepos;
pub mod lexer;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenT {
  //SKIP
  WHITESPACE,
  NEWLINE,
  COMMENT,

  //Keywords
  MOV,
  NOP,
  TYPE, //=> (byte, word, dword, float) => every type is signed we will automaticly insert unsigneds

  //Values
  GPREGISTER,
  SPECIALREGISTER,
  INT,
  FLOAT,
  STRING,
}

impl TokenT {
  pub fn is_skip(&self) -> bool {
    match self {
      TokenT::WHITESPACE => true,
      TokenT::NEWLINE => true,
      TokenT::COMMENT => true,
      _ => false,
    }
  }

  pub fn with_regex(&self) -> Vec<(Regex, TokenT)> {
    match self {
      TokenT::WHITESPACE => vec![(Regex::new(r"^\s+").unwrap(), TokenT::WHITESPACE)],
      TokenT::NEWLINE => vec![(Regex::new(r"^\n+").unwrap(), TokenT::NEWLINE)],
      TokenT::COMMENT => vec![(Regex::new(r"^//.*").unwrap(), TokenT::COMMENT)],

      TokenT::NOP => vec![(Regex::new(r"^nop").unwrap(), TokenT::NOP)],
      TokenT::MOV => vec![(Regex::new(r"^mov").unwrap(), TokenT::MOV)],
      TokenT::TYPE => vec![(
        Regex::new(r"^(byte|word|dword|float)").unwrap(),
        TokenT::TYPE,
      )],

      TokenT::INT => vec![(Regex::new(r"^\d+").unwrap(), TokenT::INT)],
      TokenT::FLOAT => vec![(Regex::new(r"^\d+\.\d+").unwrap(), TokenT::FLOAT)],
      TokenT::STRING => vec![(Regex::new(r#"^\".*\""#).unwrap(), TokenT::STRING)],
      TokenT::GPREGISTER => vec![(
        Regex::new(r"^(r1|r2|r3|r4|r5)").unwrap(),
        TokenT::GPREGISTER,
      )],
      TokenT::SPECIALREGISTER => vec![(
        Regex::new(r"^(ip|sp|bp|flags)").unwrap(),
        TokenT::SPECIALREGISTER,
      )],
      _ => vec![],
    }
  }

  pub fn is_value_required(&self) -> bool {
    match self {
      TokenT::WHITESPACE => false,
      TokenT::NEWLINE => false,
      TokenT::COMMENT => false,
      TokenT::NOP => false,
      _ => true,
    }
  }
}
