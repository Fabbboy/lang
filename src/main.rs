mod lexer;
use lexer::lexer::Lexer;

mod parser;
use parser::parser::Parser;

fn main() {
  let mut lexer = Lexer::new();
  lexer.lex("mov rax, 1\nnop");

  /*let mut parser = Parser::new(lexer.tokens);
  println!("{}", parser.parse());*/
}
