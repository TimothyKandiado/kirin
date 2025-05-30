use crate::{Register, VM};
use instructions::{Instruction, InstructionDecoder};

impl VM {
    #[inline(always)]
    pub(crate) fn set_register(&mut self, destination: Instruction, value: Register) {
        self.registers[destination as usize + self.register_offset] = value
    }

    #[inline(always)]
    pub(crate) fn set_int_in_register(&mut self, destination: Instruction, value: i64) {
        let value = value as u64;

        self.set_register(destination, value)
    }

    #[inline(always)]
    pub(crate) fn set_float_in_register(&mut self, destination: Instruction, value: f64) {
        let value = value.to_bits();

        self.set_register(destination, value)
    }

    #[inline(always)]
    pub(crate) fn get_register(&mut self, source: Instruction) -> Register {
        self.registers[source as usize + self.register_offset]
    }

    #[inline(always)]
    pub(crate) fn allocate_registers(&mut self, instruction: Instruction) {
        let count = InstructionDecoder::decode_16bit_value(instruction) as usize;
        let new_len = self.registers.len() + count;

        self.registers.resize(new_len, 0);
    }

    #[inline(always)]
    pub(crate) fn deallocate_registers(&mut self, instruction: Instruction) {
        let count = InstructionDecoder::decode_16bit_value(instruction) as usize;

        let new_len = self.registers.len() - count;
        self.registers.resize(new_len, 0);
    }

    #[inline(always)]
    pub(crate) fn move_register(&mut self, destination: Instruction, source: Instruction) {
        let value = self.get_register(source);

        self.set_register(destination, value);
    }
}
