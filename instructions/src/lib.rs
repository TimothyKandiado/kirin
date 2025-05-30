mod encoder;
mod opcodes;
mod decoder;
mod constants;

pub use opcodes::OpCode;
pub use encoder::InstructionBuilder;
pub use decoder::InstructionDecoder;

pub type Instruction = u32;

#[cfg(test)]
mod instruction_tests {
    use crate::{InstructionBuilder, InstructionDecoder, OpCode};

    #[test]
    fn test_opcode_encoding_decoding () {
        let opcodes = vec![
            OpCode::None,
            OpCode::LoadFloat64,
            OpCode::AddInt,
            OpCode::ModFloat,
            OpCode::DivInt
        ];

        for opcode in opcodes {
            let instruction = InstructionBuilder::new()
                .set_opcode(opcode)
                .set_destination_register(opcode as u32)
                .set_source1_register(opcode as u32)
                .set_source2_register(opcode as u32)
                .build();

            let decoded_opcode = InstructionDecoder::decode_opcode(instruction);

            assert_eq!(opcode as u8, decoded_opcode);
        }
    }

    #[test]
    fn test_destination_encoding_decoding() {
        for value in 0..256u32 {
            let instruction = InstructionBuilder::load_16bit_value(value, value);

            let decoded_destination = InstructionDecoder::decode_destination(instruction);

            assert_eq!(value, decoded_destination);
        }
    }

    #[test]
    fn test_source1_encoding_decoding() {
        for value in 0..256u32 {
            let instruction = InstructionBuilder::binary_operation(OpCode::AddInt, value, value, value);

            let decoded_value = InstructionDecoder::decode_source_1(instruction);

            assert_eq!(value, decoded_value);
        }
    }

    #[test]
    fn test_source2_encoding_decoding() {
        for value in 0..256u32 {
            let instruction = InstructionBuilder::binary_operation(OpCode::AddInt, value, value, value);

            let decoded_value = InstructionDecoder::decode_source_2(instruction);

            assert_eq!(value, decoded_value);
        }
    }

    #[test]
    fn test_16bit_value_encoding_decoding() {
        let max = u16::MAX as u32;
        for value in 0..=max {
            let instruction = InstructionBuilder::new()
                .set_opcode(OpCode::LoadInt16)
                .set_destination_register(0)
                .set_16bit_value(value)
                .build();

            let decoded_value = InstructionDecoder::decode_16bit_value(instruction);

            assert_eq!(value, decoded_value);
        }
    }
}