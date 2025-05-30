use crate::VM;
use instructions::{Instruction, InstructionDecoder};

impl VM {
    #[inline]
    pub(crate) fn load_int16(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let value = InstructionDecoder::decode_16bit_int(instruction);

        self.set_int_in_register(destination, value as i64);
    }
}
