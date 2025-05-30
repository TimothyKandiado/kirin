use crate::Instruction;
use crate::constants::{
    DESTINATION_MASK, EIGHT_BIT_MASK, OPCODE_MASK, SIXTEEN_BIT_MASK, SOURCE_1_MASK, SOURCE_2_MASK,
};
use crate::opcodes::OpCode;

pub struct InstructionBuilder {
    instruction: Instruction,
}

impl InstructionBuilder {
    pub fn new() -> Self {
        Self { instruction: 0 }
    }

    pub fn set_opcode(self, opcode: OpCode) -> InstructionBuilder {
        let opcode = (opcode as u32) & EIGHT_BIT_MASK; // make sure it's an 8 bit value
        let shifted = opcode << 24; // shift to occupy left most 8 bits

        let instruction = (self.instruction & !OPCODE_MASK) | shifted;
        Self { instruction }
    }

    pub fn set_destination_register(self, destination: Instruction) -> InstructionBuilder {
        let destination = destination & EIGHT_BIT_MASK; // make sure it's an 8 bit value
        let shifted = destination << 16;

        let instruction = (self.instruction & !DESTINATION_MASK) | shifted;
        Self { instruction }
    }

    pub fn set_source2_register(self, source: Instruction) -> InstructionBuilder {
        let source = source & EIGHT_BIT_MASK; // make sure it's an 8 bit value
        let shifted = source << 8;

        let instruction = (self.instruction & !SOURCE_2_MASK) | shifted;
        Self { instruction }
    }

    pub fn set_source1_register(self, source: Instruction) -> InstructionBuilder {
        let source = source & EIGHT_BIT_MASK; // make sure it's an 8 bit value

        let instruction = (self.instruction & !SOURCE_1_MASK) | source;
        Self { instruction }
    }

    pub fn set_16bit_value(self, value: Instruction) -> InstructionBuilder {
        let value = value & SIXTEEN_BIT_MASK; // make sure it's a sixteen bit value

        let instruction = (self.instruction & !SIXTEEN_BIT_MASK) | value;
        Self { instruction }
    }

    pub fn set_16bit_int(self, value: i16) -> InstructionBuilder {
        let value = value as u16;

        self.set_16bit_value(value as u32)
    }

    pub fn build(self) -> Instruction {
        self.instruction
    }

    pub fn simple(opcode: OpCode) -> Instruction {
        InstructionBuilder::new().set_opcode(opcode).build()
    }

    pub fn load_16bit_value(destination: Instruction, value: Instruction) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(OpCode::LoadInt16)
            .set_destination_register(destination)
            .set_16bit_value(value)
            .build()
    }

    pub fn load_16bit_int(destination: Instruction, value: i16) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(OpCode::LoadInt16)
            .set_destination_register(destination)
            .set_16bit_int(value)
            .build()
    }

    pub fn binary_operation(
        opcode: OpCode,
        destination: Instruction,
        source1: Instruction,
        source2: Instruction,
    ) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(opcode)
            .set_destination_register(destination)
            .set_source1_register(source1)
            .set_source2_register(source2)
            .build()
    }

    pub fn add_int(
        destination: Instruction,
        source1: Instruction,
        source2: Instruction,
    ) -> Instruction {
        Self::binary_operation(OpCode::AddInt, destination, source1, source2)
    }

    pub fn cast(opcode: OpCode, destination: Instruction, source: Instruction) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(opcode)
            .set_destination_register(destination)
            .set_source1_register(source)
            .build()
    }

    pub fn print_any(source: Instruction) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(OpCode::PrintAny)
            .set_source1_register(source)
            .build()
    }

    pub fn print_char(c: char) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(OpCode::PrintChar)
            .set_source1_register(c as u32)
            .build()
    }

    pub fn allocate_registers(count: Instruction) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(OpCode::AllocReg)
            .set_16bit_value(count)
            .build()
    }

    pub fn deallocate_registers(count: Instruction) -> Instruction {
        InstructionBuilder::new()
            .set_opcode(OpCode::DeallocReg)
            .set_16bit_value(count)
            .build()
    }
}
