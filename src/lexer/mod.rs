use regex::Regex;

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
  BYTE,
  INTEGER,
  FLOAT,
  STRING,

  //Symbols
  EQUALS,
  ASSIGN,
}

impl TokenT {
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
      TokenT::BYTE => vec![(Regex::new(r"^b'[a-zA-Z0-9]'").unwrap(), TokenT::BYTE)],
      TokenT::INTEGER => vec![(Regex::new(r"^[0-9]+").unwrap(), TokenT::INTEGER)],
      TokenT::FLOAT => vec![(Regex::new(r"^[0-9]+\.[0-9]+").unwrap(), TokenT::FLOAT)],
      TokenT::STRING => vec![(Regex::new(r#"^"([^"]|\\")*""#).unwrap(), TokenT::STRING)],

      TokenT::EQUALS => vec![(Regex::new(r"^==").unwrap(), TokenT::EQUALS)],
      TokenT::ASSIGN => vec![(Regex::new(r"^(=|\\+=|-=|\\*=|/=)").unwrap(), TokenT::ASSIGN)],
    }
  }
}
