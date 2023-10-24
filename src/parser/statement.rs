pub mod assign_statement;

#[derive(Debug, PartialEq)]
pub enum Statement {
  Assign(assign_statement::AssignStatement),
}
