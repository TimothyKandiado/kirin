use crate::constants::{DESTINATION_MASK, SIXTEEN_BIT_MASK, SOURCE_1_MASK, SOURCE_2_MASK};
use crate::Instruction;

pub struct InstructionDecoder {}

impl InstructionDecoder {
    pub fn decode_opcode(instruction: Instruction) -> u8 {
        let instruction = instruction >> 24; // shift opcode bits to the right

        instruction as u8
    }

    pub fn decode_destination(instruction: Instruction) -> Instruction {
        let instruction = (instruction & DESTINATION_MASK) >> 16;

        instruction
    }

    pub fn decode_source_1(instruction: Instruction) -> Instruction {
        let instruction = instruction & SOURCE_1_MASK;

        instruction
    }

    pub fn decode_source_2(instruction: Instruction) -> Instruction {
        let instruction = (instruction & SOURCE_2_MASK) >> 8;

        instruction
    }

    pub fn decode_16bit_value(instruction: Instruction) -> Instruction {
        let instruction = instruction & SIXTEEN_BIT_MASK;

        instruction
    }
}