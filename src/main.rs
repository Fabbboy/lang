mod lexer;
use lexer::lexer::Lexer;

mod parser;
use parser::parser::Parser;

fn main() {
  let mut lexer = Lexer::new();
  lexer.lex("str asd = \"hello world\"");

  let mut parser = Parser::new(lexer.tokens);
  parser.parse();
  println!("{:#?}", parser.module);
}
