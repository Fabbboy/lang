mod lexer;
use lexer::lexer::Lexer;

mod parser;
use parser::parser::Parser;

fn main() {
  let mut lexer = Lexer::new();
  lexer.lex("byte asd = 2\n$asd = 4");

  let mut parser = Parser::new(lexer.tokens);
  parser.parse();
  println!("{:#?}", parser.module);
}
