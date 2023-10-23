pub mod assign_statement;

#[derive(Debug)]
pub enum Statement {
  Assign(assign_statement::AssignStatement),
}
