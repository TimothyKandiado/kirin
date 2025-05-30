use crate::VM;
use instructions::{Instruction, InstructionDecoder};

impl VM {
    #[inline]
    pub(crate) fn add_int(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = self.get_register(source1) as i64;
        let second = self.get_register(source2) as i64;

        let result = first + second;

        self.set_int_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn sub_int(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = self.get_register(source1) as i64;
        let second = self.get_register(source2) as i64;

        let result = first - second;

        self.set_int_in_register(destination, result);
    }
}
