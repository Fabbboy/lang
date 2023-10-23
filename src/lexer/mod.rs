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
  TYPE,
  MUT,

  //Value
  IDENTIFIER,
  INTEGER,
  FLOAT,
  STRING,

  //Symbols
  EQUALS,
  ASSIGN,
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
      TokenT::WHITESPACE => vec![(Regex::new(r"^ +").unwrap(), TokenT::WHITESPACE)],
      TokenT::NEWLINE => vec![(Regex::new(r"^\n+").unwrap(), TokenT::NEWLINE)],
      TokenT::COMMENT => vec![(Regex::new(r"^%.*%").unwrap(), TokenT::COMMENT)],

      TokenT::TYPE => vec![(
        Regex::new(r"^(byte|int|float|str|void)\b").unwrap(),
        TokenT::TYPE,
      )],
      TokenT::MUT => vec![(Regex::new(r"^mut").unwrap(), TokenT::MUT)],

      TokenT::IDENTIFIER => vec![(
        Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
        TokenT::IDENTIFIER,
      )],
      TokenT::INTEGER => vec![(Regex::new(r"^[0-9]+").unwrap(), TokenT::INTEGER)],
      TokenT::FLOAT => vec![(Regex::new(r"^[0-9]+\.[0-9]+").unwrap(), TokenT::FLOAT)],
      TokenT::STRING => vec![(Regex::new(r#"^"([^"]|\\")*""#).unwrap(), TokenT::STRING)],

      TokenT::EQUALS => vec![(Regex::new(r"^==").unwrap(), TokenT::EQUALS)],
      TokenT::ASSIGN => vec![(Regex::new(r"^(=|\\+=|-=|\\*=|/=)").unwrap(), TokenT::ASSIGN)],
      _ => vec![],
    }
  }

  pub fn is_value_required(&self) -> bool {
    match self {
      TokenT::WHITESPACE => false,
      TokenT::NEWLINE => false,
      TokenT::COMMENT => false,
      TokenT::EQUALS => false,
      TokenT::MUT => false,
      _ => true,
    }
  }
}
