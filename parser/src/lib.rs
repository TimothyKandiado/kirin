pub mod expressions;
pub mod span;
pub mod statements;
pub mod value;
pub mod visitor;

use expressions::{
    Assign, Binary, BinaryOp, Call, Expression, Grouping, Literal, Unary, UnaryOp, Variable,
};

use errors::{KirinError, SpannedError};
use scanner::{Token, TokenSpan, TokenType};
use span::AstSpan;
use statements::{Statement, VariableDeclaration};
use value::ParsedValue;

const MAX_PARAMETERS: usize = 8;

pub struct Parser {
    tokens: Vec<Token>,
    filename: Option<String>,
    current: usize,
}

pub fn parse_ast(
    tokens: Vec<Token>,
    filename: Option<String>,
) -> Result<Vec<Statement>, Vec<KirinError>> {
    let mut parser = Parser::new(tokens, filename);

    parser.parse_all()
}

impl Parser {
    fn new(tokens: Vec<Token>, filename: Option<String>) -> Parser {
        Self {
            tokens,
            filename,
            current: 0,
        }
    }

    fn parse_all(&mut self) -> Result<Vec<Statement>, Vec<KirinError>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {
            let result = self.declaration();

            match result {
                Ok(stmt) => statements.push(stmt),
                Err(error) => {
                    errors.push(error);
                    self.synchronize();
                }
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement, KirinError> {
        if self.match_tokens(&[TokenType::NewLine]) {
            // skip trailing new line tokens
            self.declaration()
        } else if self.match_tokens(&[TokenType::Let]) {
            self.var_declaration()
        } else if self.check_next(TokenType::ColonEqual) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement, KirinError> {
        let name = self.consume(TokenType::Identifier)?.clone();

        let mut initializer = None;

        if self.match_tokens(&[TokenType::Equal, TokenType::ColonEqual]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::NewLine)?;

        let span = AstSpan::from_token_span(name.span, self.filename.clone());
        Ok(Statement::VarDeclaration(VariableDeclaration {
            name: name.lexeme.clone(),
            initializer,
            span,
        }))
    }

    fn statement(&mut self) -> Result<Statement, KirinError> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Statement, KirinError> {
        let expression = self.expression()?;

        self.consume(TokenType::NewLine)?;
        Ok(Statement::ExpressionStatement(expression))
    }

    pub fn expression(&mut self) -> Result<Expression, KirinError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, KirinError> {
        let expression = self.or()?;

        if self.match_tokens(&[TokenType::Equal]) {
            let equals = self.previous().clone();

            let span = AstSpan::from_token_span(equals.span, self.filename.clone());

            let value = self.assignment()?;

            if let Expression::Variable(variable) = &expression {
                let name = variable.name.clone();
                return Ok(Expression::Assign(Box::new(Assign::new(name, value, span))));
            }

            return Err(self.error_from_token_span(equals.span, "invalid assignment target"));
        }

        Ok(expression)
    }

    fn or(&mut self) -> Result<Expression, KirinError> {
        let expression = self.and()?;

        if self.match_tokens(&[TokenType::Or]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.and()?;

            let binary =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn and(&mut self) -> Result<Expression, KirinError> {
        let expression = self.equality()?;

        if self.match_tokens(&[TokenType::And]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.equality()?;

            let binary =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression, KirinError> {
        let expression = self.comparison()?;

        if self.match_tokens(&[TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.comparison()?;

            let binary =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, KirinError> {
        let expression: Expression = self.addition()?;

        if self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.addition()?;

            let binary =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)));
            return Ok(binary);
        }

        Ok(expression)
    }

    fn addition(&mut self) -> Result<Expression, KirinError> {
        let mut expression = self.multiplication()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.multiplication()?;

            expression =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)))
        }

        Ok(expression)
    }

    fn multiplication(&mut self) -> Result<Expression, KirinError> {
        let mut expression = self.power()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash, TokenType::Percent]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.unary()?;

            expression =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)))
        }

        Ok(expression)
    }

    fn power(&mut self) -> Result<Expression, KirinError> {
        let mut expression = self.unary()?;

        while self.match_tokens(&[TokenType::Caret]) {
            let operator_token = self.previous();

            let operator = BinaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.unary()?;

            expression =
                Expression::Binary(Box::new(Binary::new(expression, right, operator, span)))
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, KirinError> {
        if self.match_tokens(&[TokenType::Minus, TokenType::Not]) {
            let operator_token = self.previous();

            let operator = UnaryOp::from_token(operator_token)?;
            let span = AstSpan::from_token_span(operator_token.span, self.filename.clone());

            let right = self.unary()?;

            return Ok(Expression::Unary(Box::new(Unary::new(
                right, operator, span,
            ))));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expression, KirinError> {
        let expression = self.primary()?;

        if self.match_tokens(&[TokenType::LeftParen]) {
            return self.finish_call(expression);
        }

        Ok(expression)
    }

    fn get_arguments(&mut self) -> Result<Vec<Expression>, KirinError> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() > MAX_PARAMETERS {
                    let previous = self.previous().clone();
                    return Err(self.error_from_token_span(previous.span, "Too many arguments"));
                }

                arguments.push(self.expression()?);
                if !self.match_tokens(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        Ok(arguments)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, KirinError> {
        let arguments = self.get_arguments()?;

        let paren = self.consume(TokenType::RightParen)?;

        let span = AstSpan::from_token_span(paren.span, self.filename.clone());
        Ok(Expression::Call(Box::new(Call::new(
            callee, span, arguments,
        ))))
    }

    fn primary(&mut self) -> Result<Expression, KirinError> {
        // handle identifiers and function calls
        if self.match_tokens(&[TokenType::Identifier]) {
            let token = self.previous().clone();

            let span = AstSpan::from_token_span(token.span, self.filename.clone());

            return Ok(Expression::Variable(Box::new(Variable::new(
                token.lexeme,
                span,
            ))));
        }

        // Handle literals
        if self.match_tokens(&[
            TokenType::Number,
            TokenType::String,
            TokenType::True,
            TokenType::False,
            TokenType::None,
        ]) {
            let token = self.previous().clone();

            let value = ParsedValue::from_token(&token)?;
            let span = AstSpan::from_token_span(token.span, self.filename.clone());

            return Ok(Expression::Literal(Literal::new(value, span)));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.expression()?;
            let token = self.consume(TokenType::RightParen)?;

            return Ok(Expression::Grouping(Box::new(Grouping::new(
                expression,
                AstSpan::from_token_span(token.span, self.filename.clone()),
            ))));
        }

        if self.match_tokens(&[TokenType::NewLine]) {
            return self.primary();
        }

        let current = self.peek().clone();
        Err(self.error_from_token_span(
            current.span,
            &format!("expected Expression but found `{:?}`", current.token_type),
        ))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::NewLine {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Return => return,

                _ => {}
            }

            self.advance();
        }
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
        for &token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token, KirinError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let previous = self.previous().clone();
        Err(self.error_from_token_span(
            previous.span,
            &format!("expected {:?}, got {:?}", token_type, previous.token_type),
        ))
    }

    /// Check if the current token is of the given type
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn check_next(&self, token_type: TokenType) -> bool {
        let next = self.peek_next();
        if let Some(token) = next {
            return token.token_type == token_type;
        }
        false
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Get the current token
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Get the next token
    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    /// Get the previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn error_from_token_span(&mut self, span: TokenSpan, message: &str) -> KirinError {
        KirinError::Parse(SpannedError {
            line: span.line,
            column: span.column,
            message: message.to_string(),
        })
    }
}
