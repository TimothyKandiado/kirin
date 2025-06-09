mod frame;
mod handlers;
mod opcodes;
mod program;
mod register;

use errors::KirinError;
use instructions::{Instruction, InstructionDecoder};
use opcodes::*;

use crate::frame::Frame;
pub use program::{Program, ProgramConstant, ProgramMetadata};
pub use register::Register;

#[repr(u8)]
enum VmStatus {
    Running,
    Halted,
    Error,
}

pub struct VM {
    instructions: Vec<Instruction>,
    constants: Vec<ProgramConstant>,
    registers: Vec<Register>,
    frames: Vec<Frame>,
    instruction_pointer: usize,
    return_register: Register,
    register_offset: usize,
    status: VmStatus,
    error: Option<String>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            registers: Vec::new(),
            instruction_pointer: 0,
            return_register: 0,
            status: VmStatus::Halted,
            error: None,
            register_offset: 0,
            frames: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: Program) -> Result<(), KirinError> {
        if program.instructions.len() == 0 {
            return Ok(());
        }

        let last_instruction = program.instructions[program.instructions.len() - 1];
        if InstructionDecoder::decode_opcode(last_instruction) != OP_HALT {
            return Err(KirinError::General(
                "program does not end with halt instruction".to_string(),
            ));
        }

        let mut instructions = program.instructions.clone();
        let mut constants = program.constants.clone();

        self.instructions.append(&mut instructions);
        self.constants.append(&mut constants);

        Ok(())
    }

    pub fn start_with_offset(&mut self, offset: usize) -> Result<(), KirinError> {
        self.instruction_pointer += offset;
        self.status = VmStatus::Running;

        self.start_execution()?;

        Ok(())
    }

    fn start_execution(&mut self) -> Result<(), KirinError> {
        loop {
            match self.status {
                VmStatus::Running => {
                    let instruction = self.get_next_instruction();
                    self.execute_instruction(instruction);
                }

                VmStatus::Halted => {
                    break;
                }

                VmStatus::Error => {
                    let error = self
                        .error
                        .clone()
                        .unwrap_or("error flag was set".to_string());
                    return Err(KirinError::General(error));
                }
            }
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        let opcode = InstructionDecoder::decode_opcode(instruction);

        match opcode {
            OP_NONE => {}

            // Loading
            OP_LOAD_INT16 => self.load_int16(instruction),

            // Arithmetic
            OP_ADD_INT => self.add_int(instruction),
            OP_SUB_INT => self.sub_int(instruction),
            OP_MUL_INT => self.mul_int(instruction),
            OP_DIV_INT => self.div_int(instruction),
            OP_MOD_INT => self.mod_int(instruction),
            OP_POW_INT => self.pow_int(instruction),
            OP_ADD_FLOAT => self.add_float(instruction),
            OP_SUB_FLOAT => self.sub_float(instruction),
            OP_MUL_FLOAT => self.mul_float(instruction),
            OP_DIV_FLOAT => self.div_float(instruction),
            OP_MOD_FLOAT => self.mod_float(instruction),
            OP_POW_FLOAT => self.pow_float(instruction),

            // Allocations
            OP_ALLOC_REG => self.allocate_registers(instruction),
            OP_DEALLOC_REG => self.deallocate_registers(instruction),

            // Casting
            OP_INT_TO_ANY => self.cast_int_to_any(instruction),
            OP_FLOAT_TO_ANY => self.cast_float_to_any(instruction),
            OP_INT_TO_FLOAT => self.cast_int_to_float(instruction),
            OP_FLOAT_TO_INT => self.cast_float_to_int(instruction),

            // Control flow
            OP_RETURN => self.do_return(instruction),
            OP_HALT => {
                self.status = VmStatus::Error;
                self.error = Some("halt instruction encountered".to_string())
            }

            // Library
            OP_PRINT_ANY => self.print_any(instruction),
            OP_PRINT_CHAR => self.print_char(instruction),

            _ => {
                self.status = VmStatus::Error;
                self.error = Some(format!("unknown instruction encountered: {:?}", opcode));
            }
        }
    }

    fn get_next_instruction(&mut self) -> Instruction {
        self.instruction_pointer += 1;
        self.instructions[self.instruction_pointer - 1]
    }
}
