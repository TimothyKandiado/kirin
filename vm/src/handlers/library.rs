use crate::VM;
use instructions::{Instruction, InstructionDecoder};
use types::KirinType;

impl VM {
    pub(crate) fn print_any(&mut self, instruction: Instruction) {
        let source = InstructionDecoder::decode_source_1(instruction);

        let type_tag = self.get_register(source);
        let value = self.get_register(source + 1);

        let type_val = KirinType::from_u8(type_tag as u8);

        if let Some(kind) = type_val {
            match kind {
                KirinType::Int => print!("{}", value as i64),
                KirinType::Float => print!("{}", f64::from_bits(value)),
                _ => print!("Unsupported type: {:x}", value),
            }
        } else {
            print!("Unsupported type: {:x} {:x}", type_tag, value)
        }
    }

    pub(crate) fn print_char(&mut self, instruction: Instruction) {
        let value = InstructionDecoder::decode_source_1(instruction);

        print!("{}", value as u8 as char);
    }
}
