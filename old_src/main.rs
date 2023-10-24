

fn main() {
  let mut lexer = Lexer::new();
  lexer.lex("byte isTrue? = 1\n$isTrue = 0");

  let mut parser = Parser::new(lexer.tokens);
  parser.parse();
  println!("{:#?}", parser.module);
}
