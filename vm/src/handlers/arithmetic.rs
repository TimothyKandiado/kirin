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

    #[inline]
    pub(crate) fn mul_int(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = self.get_register(source1) as i64;
        let second = self.get_register(source2) as i64;

        let result = first * second;

        self.set_int_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn div_int(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = self.get_register(source1) as i64;
        let second = self.get_register(source2) as i64;

        let result = first / second;

        self.set_int_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn mod_int(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = self.get_register(source1) as i64;
        let second = self.get_register(source2) as i64;

        let result = first % second;

        self.set_int_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn pow_int(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = self.get_register(source1) as i64;
        let second = self.get_register(source2) as i64;

        let result = first.pow(second as u32);

        self.set_int_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn add_float(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = f64::from_bits(self.get_register(source1));
        let second = f64::from_bits(self.get_register(source2));

        let result = first + second;

        self.set_float_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn sub_float(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = f64::from_bits(self.get_register(source1));
        let second = f64::from_bits(self.get_register(source2));

        let result = first - second;

        self.set_float_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn mul_float(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = f64::from_bits(self.get_register(source1));
        let second = f64::from_bits(self.get_register(source2));

        let result = first * second;

        self.set_float_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn div_float(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = f64::from_bits(self.get_register(source1));
        let second = f64::from_bits(self.get_register(source2));

        let result = first / second;

        self.set_float_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn mod_float(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = f64::from_bits(self.get_register(source1));
        let second = f64::from_bits(self.get_register(source2));

        let result = first % second;

        self.set_float_in_register(destination, result);
    }

    #[inline]
    pub(crate) fn pow_float(&mut self, instruction: Instruction) {
        let destination = InstructionDecoder::decode_destination(instruction);
        let source1 = InstructionDecoder::decode_source_1(instruction);
        let source2 = InstructionDecoder::decode_source_2(instruction);

        let first = f64::from_bits(self.get_register(source1));
        let second = f64::from_bits(self.get_register(source2));

        let result = first.powf(second);

        self.set_float_in_register(destination, result);
    }
}
