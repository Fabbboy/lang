pub mod assign_statement;

#[derive(Debug)]
pub enum Statement {
  Nop(),
  Assign(assign_statement::AssignStatement),
}
