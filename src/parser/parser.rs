use crate::lexer::{token::Token, TokenT};

use super::{
  expression::{
    type_expression::{Type, TypeExpression},
    value_expression::{
      float_value::FloatValue, int_value::IntValue, string_value::StringValue, ValueExpression,
    },
    variable_expression::VariableExpression,
    Expression,
  },
  module::Module,
  statement::{assign_statement::AssignStatement, Statement},
};

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

  fn consume_if_or(&mut self, token_type: TokenT, token_type2: TokenT) -> bool {
    if self.peek(0).token == token_type || self.peek(0).token == token_type2 {
      self.consume();
      return true;
    }
    false
  }

  fn get_at(&self, n: usize) -> Token {
    if n >= self.tokens.len() {
      return Token::zero();
    }
    self.tokens[n].clone()
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
    let current_token = self.consume();
    match current_token.token {
      TokenT::TYPE => {
        if let Some(stmt) = self.parse_assignment() {
          self.module.add_statement(stmt);
        } else {
          eprintln!("Error: Failed to parse assignment");
        }
      }
      _ => {
        eprintln!(
          "Error: Unexpected token {:?} at {:?}",
          current_token.token, current_token.pos
        );
      }
    }
  }

  fn parse_assignment(&mut self) -> Option<Statement> {
    self.pos -= 1;
    let type_expr = self.parse_type();
    if type_expr.is_none() {
      eprintln!("Error: Expected type at {:?}", self.peek(0).pos);
      return None;
    }
    let type_expr = type_expr.unwrap();
    self.consume();
    let t = self.consume_if_or(TokenT::IDENTIFIER, TokenT::MUT);
    let mut mutable = false;
    let name = if t {
      match self.get_at(self.pos - 1).token {
        TokenT::MUT => {
          mutable = true;
          self.consume_if(TokenT::IDENTIFIER);
          self.get_at(self.pos - 1).value.unwrap()
        }
        TokenT::IDENTIFIER => self.get_at(self.pos - 1).value.unwrap(),

        _ => {
          eprintln!(
            "Error: Expected IDENTIFIER or MUT at {:?}",
            self.peek(0).pos
          );
          return None;
        }
      }
    } else {
      eprintln!("Error: Expected IDENTIFIER at {:?}", self.peek(0).pos);
      return None;
    };

    let variable_expr = Expression::Variable(VariableExpression::new(name, Box::from(type_expr)));

    if !self.consume_if(TokenT::ASSIGN) {
      eprintln!("Error: Expected ASSIGN at {:?}", self.peek(0).pos);
      return None;
    }

    let value = self.parse_value();
    if value.is_none() {
      eprintln!("Error: Expected value at {:?}", self.peek(0).pos);
      return None;
    }
    let value = value.unwrap();
    self.consume();

    Some(Statement::Assign(AssignStatement::new(
      variable_expr,
      value,
      mutable,
    )))
  }

  fn parse_value(&self) -> Option<Expression> {
    let current_token = self.peek(0);
    match current_token.token {
      TokenT::INTEGER => {
        let value = self.peek(0).value.as_ref().unwrap().parse::<i32>();
        if value.is_err() {
          eprintln!("Error: Failed to parse integer at {:?}", self.peek(0).pos);
          return None;
        }
        return Some(Expression::Value(ValueExpression::Int(IntValue::new(
          value.unwrap(),
        ))));
      }
      TokenT::FLOAT => {
        let value = self.peek(0).value.as_ref().unwrap().parse::<f32>();
        if value.is_err() {
          eprintln!("Error: Failed to parse float at {:?}", self.peek(0).pos);
          return None;
        }
        return Some(Expression::Value(ValueExpression::Float(FloatValue::new(
          value.unwrap(),
        ))));
      }
      TokenT::STRING => {
        let value = self.peek(0).value.as_ref().unwrap().clone();
        return Some(Expression::Value(ValueExpression::String(
          StringValue::new(value),
        )));
      }
      TokenT::TYPE => {
        return Some(Expression::Value(ValueExpression::Void));
      }
      TokenT::IDENTIFIER => {
        let value = self.peek(0).value.as_ref().unwrap().clone();
        return Some(Expression::Variable(VariableExpression::new(
          value.to_string(),
          Box::from(Expression::Type(TypeExpression::new(Type::Void))),
        )));
      }

      _ => todo!("Not implemented for {:?}", current_token.token),
    }
  }

  fn parse_type(&self) -> Option<Expression> {
    match self.peek(0).token {
      TokenT::TYPE => match self.peek(0).value.as_ref().unwrap().as_str() {
        "byte" => Some(Expression::Type(TypeExpression::new(Type::Byte))),
        "int" => Some(Expression::Type(TypeExpression::new(Type::Int))),
        "float" => Some(Expression::Type(TypeExpression::new(Type::Float))),
        "str" => Some(Expression::Type(TypeExpression::new(Type::String))),
        "void" => Some(Expression::Type(TypeExpression::new(Type::Void))),
        _ => None,
      },
      _ => None,
    }
  }
}
