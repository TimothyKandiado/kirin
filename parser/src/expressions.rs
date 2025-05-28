pub use crate::expressions::assignment::Assign;
pub use crate::expressions::binary::{Binary, BinaryOp};
pub use crate::expressions::call::Call;
pub use crate::expressions::grouping::Grouping;
pub use crate::expressions::literal::Literal;
pub use crate::expressions::unary::{Unary, UnaryOp};
pub use crate::expressions::variable::Variable;

use crate::visitor::ExpressionVisitor;

mod assignment;
mod binary;
mod call;
mod grouping;
mod literal;
mod unary;
mod variable;

#[derive(Debug, Clone)]
pub enum Expression {
    /// left operator right
    Binary(Box<Binary>),
    Unary(Box<Unary>),
    Literal(Literal),
    Grouping(Box<Grouping>),
    Variable(Box<Variable>),
    Assign(Box<Assign>),
    Call(Box<Call>),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &mut impl ExpressionVisitor<Output = T>) -> T {
        match self {
            Self::Binary(binary) => binary.accept(visitor),
            Self::Unary(unary) => unary.accept(visitor),
            Self::Grouping(grouping) => grouping.accept(visitor),
            Self::Literal(literal) => literal.accept(visitor),
            Self::Variable(variable_expression) => variable_expression.accept(visitor),
            Self::Assign(assign) => assign.accept(visitor),
            Self::Call(math_function) => math_function.accept(visitor),
        }
    }
}
