use super::expression::Expression;
use super::parameter::Parameter;


#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment {
        identifier: String,
        expression: Expression
    },
    FunctionDeclaration {
        identifier: String,
        parameters: Vec<Parameter>,
        expression: Expression
    },
    ExpressionStatement {
        expression: Expression,
    },
}