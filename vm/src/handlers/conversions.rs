use crate::VM;
use instructions::{Instruction, InstructionDecoder};
use types::KirinType;

impl VM {
    pub(crate) fn cast_int_to_any(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source = InstructionDecoder::decode_source_1(instruction);

        self.set_register(destination, KirinType::Int as u64); // set type to int
        self.move_register(destination + 1, source);
    }

    pub(crate) fn cast_float_to_any(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source = InstructionDecoder::decode_source_1(instruction);

        self.set_register(destination, KirinType::Float as u64); // set type to int
        self.move_register(destination + 1, source);
    }
}
