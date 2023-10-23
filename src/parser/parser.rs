use crate::lexer::{token::Token, codepos::CodePos, TokenT};

use super::module::Module;

pub struct Parser {
  pub tokens: Vec<Token>,
  module: Module,
  pub pos: usize,
  pub codepos: CodePos,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser { 
      tokens, 
      module: Module::new(None),
      pos: 0,
      codepos: CodePos::zero(),
    }
  }
 
  pub fn get_mut_module(&mut self) -> &mut Module {
    &mut self.module
  }

  fn peek(&self , n: usize) -> Token {
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
    self.codepos = token.pos;
    token
  }

  fn consume_if(&mut self, token_type: TokenT) -> bool {
    if self.peek(0).token == token_type {
      self.consume();
      return true;
    }
    false
  }

  fn consume_if_either (&mut self, token_type: Vec<TokenT>) -> bool {
    for token in token_type {
      if self.peek(0).token == token {
        self.consume();
        return true;
      }
    }
    false
  }

  pub fn parse(&self)  {
    self.remove_all_comments();
    while self.pos < self.tokens.len() {
       self.parse_statement();
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

  fn parse_statement(&self) {
    self.consume_if_either(vec![TokenT::NEWLINE, TokenT::WHITESPACE]);
    if self.peek(0).token == TokenT::COMMENT {
      return;
    }
  }
}
