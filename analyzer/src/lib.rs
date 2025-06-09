use errors::{KirinError, SpannedError};
use parser::expressions::{Assign, Binary, Call, Expression, Grouping, Literal, Unary, Variable};
use parser::statements::{Statement, VariableDeclaration};
use parser::visitor::{ExpressionVisitor, StatementVisitor};

pub struct TypeChecker {}

impl TypeChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn infer_types(&mut self, statements: &Vec<Statement>) -> Result<Vec<Statement>, Vec<KirinError>> {
        let mut typed_statements = Vec::new();
        let mut errors = Vec::new();

        for statement in statements {
            let result = self.execute(statement);

            match result {
                Ok(statement) => typed_statements.push(statement),
                Err(error) => errors.push(error),
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(typed_statements)
    }

    fn execute(&mut self, statement: &Statement) -> Result<Statement, KirinError> {
        statement.accept(self)
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Expression, KirinError> {
        expression.accept(self)
    }
}

impl StatementVisitor for TypeChecker {
    type Output = Result<Statement, KirinError>;

    fn visit_none(&mut self) -> Self::Output {
        Ok(Statement::None)
    }

    fn visit_var_declaration(&mut self, var_declaration: &VariableDeclaration) -> Self::Output {
        Err(KirinError::Type(SpannedError{
            line: var_declaration.span.line,
            column: var_declaration.span.column,
            message: format!("type checking for binary ops not yet implemented `{:?}`", var_declaration.span.filename.clone())
        }))
    }

    fn visit_expression_statement(&mut self, expression_statement: &Expression) -> Self::Output {
        let expression = self.evaluate(expression_statement)?;

        Ok(Statement::ExpressionStatement(expression))
    }
}

impl ExpressionVisitor for TypeChecker {
    type Output = Result<Expression, KirinError>;

    fn visit_binary(&mut self, binary: &Binary) -> Self::Output {
        Err(KirinError::Type(SpannedError{
            line: binary.span.line,
            column: binary.span.column,
            message: format!("type checking for binary ops not yet implemented `{:?}`", binary.span.filename.clone())
        }))
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        Err(KirinError::Type(SpannedError{
            line: unary.span.line,
            column: unary.span.column,
            message: format!("type checking for unary ops not yet implemented `{:?}`", unary.span.filename.clone())
        }))
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output {
        let expression = self.evaluate(&grouping.expression)?;

        Ok(Expression::Grouping(Box::new(Grouping::new(expression, grouping.span.clone()))))
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Output {
        if let Some(_) = &literal.inferred_type {
            return Ok(Expression::Literal(literal.clone()));
        }

        Err(KirinError::Type(SpannedError{
            message: format!("type analyzer for literals not supported. file {}", literal.span.filename.clone().unwrap_or("".to_string())),
            line: literal.span.line,
            column: literal.span.column,
        }))
    }

    fn visit_call(&mut self, callable: &Call) -> Self::Output {
        Err(KirinError::Type(SpannedError{
            line: callable.span.line,
            column: callable.span.column,
            message: format!("type checking for calls not yet implemented `{:?}`", callable.span.filename.clone())
        }))
    }

    fn visit_variable(&mut self, variable: &Variable) -> Self::Output {
        Err(KirinError::Type(SpannedError{
            line: variable.span.line,
            column: variable.span.column,
            message: format!("type checking for variables not yet implemented `{:?}`", variable.span.filename.clone())
        }))
    }

    fn visit_assign(&mut self, assign: &Assign) -> Self::Output {
        Err(KirinError::Type(SpannedError{
            line: assign.span.line,
            column: assign.span.column,
            message: format!("type checking for assign ops not yet implemented `{:?}`", assign.span.filename.clone())
        }))
    }
}
