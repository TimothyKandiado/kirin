use instructions::{InstructionBuilder, OpCode};
use vm::{Program, VM};

fn main() {
    let program = get_program();
    let mut vm = VM::new();

    vm.load_program(program).unwrap();
    vm.start_with_offset(0).unwrap();
}

fn get_program() -> Program {
    let instructions = vec![
        InstructionBuilder::new()
            .set_opcode(OpCode::AllocReg)
            .set_16bit_value(4)
            .build(),
        InstructionBuilder::load_16bit_int(0, -2800),
        InstructionBuilder::load_16bit_int(1, 480),
        InstructionBuilder::binary_operation(OpCode::AddInt, 0, 0, 1),
        InstructionBuilder::cast(OpCode::IntToAny, 2, 0),
        InstructionBuilder::print_any(2),
        InstructionBuilder::print_char('\n'),
        InstructionBuilder::deallocate_registers(4),
        InstructionBuilder::simple(OpCode::Return),
        InstructionBuilder::simple(OpCode::Halt),
    ];

    Program::new(instructions, Vec::new())
}
