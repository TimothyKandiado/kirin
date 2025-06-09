use std::collections::HashMap;
use errors::{KirinError, SpannedError};
use instructions::{Instruction, InstructionBuilder, OpCode};
use parser::expressions::{Assign, Binary, BinaryOp, Call, Expression, Grouping, Literal, Unary, Variable};
use parser::statements::{Statement, VariableDeclaration};
use parser::visitor::{ExpressionVisitor, StatementVisitor};
use types::KirinType;
use vm::{Program, ProgramConstant};

enum Register {
    Temp(Option<KirinType>),
    Variable(Option<KirinType>),
}

struct Compiler {
    instructions: Vec<Instruction>,
    constants: Vec<ProgramConstant>,
    locals: Vec<HashMap<String, usize>>,
    registers: Vec<Register>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            locals: Vec::new(),
            registers: Vec::new(),
        }
    }

    pub fn compile(&mut self, statements: &Vec<Statement>) -> Result<(), KirinError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn emit_program(self) -> Program {
        Program::new(self.instructions, self.constants)
    }

    fn execute(&mut self, statement: &Statement) -> Result<(), KirinError> {
        statement.accept(self)?;
        Ok(())
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<(), KirinError> {
        expression.accept(self)
    }
}

impl StatementVisitor for Compiler {
    type Output = Result<(), KirinError>;

    fn visit_none(&mut self) -> Self::Output {
        Ok(())
    }

    fn visit_var_declaration(&mut self, var_declaration: &VariableDeclaration) -> Self::Output {
        todo!()
    }

    fn visit_expression_statement(&mut self, expression_statement: &Expression) -> Self::Output {
        todo!()
    }
}

impl ExpressionVisitor for Compiler {
    type Output = Result<(), KirinError>;

    fn visit_binary(&mut self, binary: &Binary) -> Self::Output {
        self.evaluate(&binary.left)?;
        self.evaluate(&binary.right)?;

        let opcode = match binary.operator {
            BinaryOp::Add => OpCode::AddInt,
            BinaryOp::Subtract => OpCode::SubInt,
            BinaryOp::Multiply => OpCode::MulInt,
            BinaryOp::Divide => OpCode::DivInt,

            _ => return Err(KirinError::Compile(SpannedError {
                line: binary.span.line,
                column: binary.span.column,
                message: format!("binary operator `{:?}` not implemented in file: `{:?}`", binary.operator, binary.span.filename),
            }))
        };

        Ok(())
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        todo!()
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Self::Output {
        todo!()
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Output {
        todo!()
    }

    fn visit_call(&mut self, callable: &Call) -> Self::Output {
        todo!()
    }

    fn visit_variable(&mut self, variable: &Variable) -> Self::Output {
        todo!()
    }

    fn visit_assign(&mut self, assign: &Assign) -> Self::Output {
        todo!()
    }
}
