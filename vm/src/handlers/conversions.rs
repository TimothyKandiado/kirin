use crate::VM;
use instructions::{Instruction, InstructionDecoder};
use types::KirinType;

impl VM {
    #[inline]
    pub(crate) fn cast_int_to_any(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source = InstructionDecoder::decode_source_1(instruction);

        self.set_register(destination, KirinType::Int as u64); // set type to int
        self.move_register(destination + 1, source);
    }

    #[inline]
    pub(crate) fn cast_float_to_any(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source = InstructionDecoder::decode_source_1(instruction);

        self.set_register(destination, KirinType::Float as u64); // set type to int
        self.move_register(destination + 1, source);
    }

    #[inline]
    pub(crate) fn cast_int_to_float(&mut self, instruction: Instruction) {
        let source = InstructionDecoder::decode_source_1(instruction);
        let value = self.get_register(source) as i64 as f64;

        self.set_float_in_register(source, value);
    }

    #[inline]
    pub(crate) fn cast_float_to_int(&mut self, instruction: Instruction) {
        let source = InstructionDecoder::decode_source_1(instruction);
        let value = f64::from_bits(self.get_register(source));

        self.set_int_in_register(source, value as i64)
    }
}
