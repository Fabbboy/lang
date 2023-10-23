use crate::lexer::{codepos::CodePos, token::Token, TokenT};

use super::{module::Module, statement::Statement};

pub struct Parser {
  pub tokens: Vec<Token>,
  pub module: Module,
  pub pos: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      module: Module::new(None),
      pos: 0,
    }
  }

  fn peek(&self, n: usize) -> Token {
    if self.pos + n >= self.tokens.len() {
      return Token::zero();
    }
    self.tokens[self.pos + n].clone()
  }

  fn consume(&mut self) -> Token {
    if self.pos >= self.tokens.len() {
      eprintln!("Error: Parser::consume() called when self.pos >= self.tokens.len()");
      return Token::zero();
    }
    let token = self.tokens[self.pos].clone();
    self.pos += 1;
    token
  }

  fn consume_if(&mut self, token_type: TokenT) -> bool {
    if self.peek(0).token == token_type {
      self.consume();
      return true;
    }
    false
  }

  fn consume_if_either(&mut self, token_type: Vec<TokenT>) -> (bool, Option<Token>) {
    for token in token_type {
      if self.peek(0).token == token {
        self.consume();
        return (true, Some(self.peek(0)));
      }
    }
    (false, None)
  }

  pub fn parse(&mut self) {
    self.remove_all_comments();
    self.filter_newline_and_whitespaces();
    while self.pos < self.tokens.len() {
      self.parse_statement();
    }
  }

  fn filter_newline_and_whitespaces(&mut self) {
    let mut i = 0;
    while i < self.tokens.len() {
      if self.tokens[i].token == TokenT::NEWLINE || self.tokens[i].token == TokenT::WHITESPACE {
        self.tokens.remove(i);
      } else {
        i += 1;
      }
    }
  }

  fn remove_all_comments(&mut self) {
    let mut i = 0;
    while i < self.tokens.len() {
      if self.tokens[i].token == TokenT::COMMENT {
        self.tokens.remove(i);
      } else {
        i += 1;
      }
    }
  }

  fn parse_statement(&mut self) {
    //if self.consume_if_either(vec![TokenT::NEWLINE, TokenT::WHITESPACE, TokenT::TYPE]) {
    if let (true, Some(token)) =
      self.consume_if_either(vec![TokenT::NEWLINE, TokenT::WHITESPACE, TokenT::TYPE])
    {
      match token.token {
        TokenT::TYPE => {
          let stmt = self.parse_assignment();
          if stmt.is_some() {
            self.module.add_statement(stmt.unwrap());
          }else {
            eprintln!("Error: Parser::parse_assignment() returned None");
          }
        }
        TokenT::NEWLINE => {}
        TokenT::WHITESPACE => {}
        _ => {
          eprintln!(
            "Error: Parser::parse_statement() called with {:?} but expected {:?} or {:?} or {:?}",
            token.token,
            TokenT::NEWLINE,
            TokenT::WHITESPACE,
            TokenT::TYPE
          );
          std::process::exit(1);
        }
      }
    } else {
      eprintln!(
        "Expected {:?} or {:?} but found {:?} at {:?}",
        TokenT::NEWLINE,
        TokenT::WHITESPACE,
        self.peek(0).token,
        self.peek(0).pos
      );
    }
  }

  fn parse_assignment(&self) -> Option<Statement> {
    None
  }
}
