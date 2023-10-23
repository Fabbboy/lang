pub mod mov_statement;

#[derive(Debug)]
pub enum Statement {
  Nop(),
  Mov(mov_statement::MovStatement),
}
