mod lexer;
use lexer::lexer::Lexer;

mod parser;
use parser::parser::Parser;

fn main() {
  let mut lexer = Lexer::new();
  lexer.lex("int asdasd = 123");
  println!("{:#?}", lexer.tokens);

  let mut parser = Parser::new(lexer.tokens);
  parser.parse();
  println!("{:#?}", parser.module);
}
