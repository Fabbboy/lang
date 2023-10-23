use crate::lexer::{codepos::CodePos, token::Token, TokenT};

#[derive(Debug, Clone)]
pub struct Lexer {
  pub tokens: Vec<Token>,
  pub pos: usize,
}

impl Lexer {
  pub fn new() -> Lexer {
    Lexer {
      tokens: Vec::new(),
      pos: 0,
    }
  }

  pub fn add_token(&mut self, token: Token) {
    self.tokens.push(token);
  }

  pub fn lex(&mut self, mut source: &str) {
    let mut line = 1;
    let mut col = 1;

    while !source.is_empty() {
      let mut matched = false;

      for token_type in &[
        TokenT::WHITESPACE,
        TokenT::NEWLINE,
        TokenT::COMMENT,
        TokenT::TYPE,
        TokenT::MUT,
        TokenT::IDENTIFIER,
        TokenT::INTEGER,
        TokenT::FLOAT,
        TokenT::STRING,
        TokenT::EQUALS,
        TokenT::ASSIGN,
      ] {
        for (regex, t) in token_type.with_regex() {
          if let Some(m) = regex.find(source) {
            let value = m.as_str();
            self.add_token(Token::new(
              t.clone(),
              CodePos::new(line, col),
              Some(value.to_string()),
            ));

            let len = value.len();
            source = &source[len..];

            if t == TokenT::NEWLINE {
              line += 1;
              col = 1;
            } else {
              col += len;
            }

            matched = true;
            break;
          }
        }

        if matched {
          break;
        }
      }

      if !matched {
        // Handle other tokens here or raise an error
        // For now, we'll just skip the character
        source = &source[1..]; // Skip one character
        col += 1;
      }
    }
  }
}
