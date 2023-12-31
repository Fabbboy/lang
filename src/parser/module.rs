use super::statement::Statement;

#[derive(Debug)]
pub struct Module {
  pub statements: Option<Vec<Statement>>,
}

impl Module {
  pub fn new(statements: Option<Vec<Statement>>) -> Self {
    Module { statements }
  }

  pub fn add_statement(&mut self, statement: Statement) {
    //self.statements.as_mut().unwrap().push(statement);
    match &mut self.statements {
      Some(statements) => statements.push(statement),
      None => self.statements = Some(vec![statement]),
    }
  }
}
