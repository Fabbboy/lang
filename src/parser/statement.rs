pub mod variable_declaration;

#[derive(Debug)]
pub enum Statement {
  Nop(),
  VariableDeclaration(variable_declaration::VariableDeclaration),
}
