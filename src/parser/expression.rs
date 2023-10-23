pub mod type_expression;
pub mod value_expression;
pub mod variable_expression;

#[derive(Debug)]
pub enum Expression {
  Value(value_expression::ValueExpression),
  Variable(variable_expression::VariableExpression),
  Type(type_expression::TypeExpression),
}
