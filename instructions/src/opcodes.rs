#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    None,

    // Load Instructions
    LoadConst,
    LoadInt16,

    // Mathematical Instructions [OpCode dest src1 src2]
    AddInt,
    AddFloat,
    SubInt,
    SubFloat,
    MulInt,
    MulFloat,
    DivInt,
    DivFloat,
    ModInt,
    ModFloat,
    PowInt,
    PowFloat,

    // Casting
    IntToAny,
    FloatToAny,
    IntToFloat,
    FloatToInt,

    // Frames
    InitFrame,
    DropFrame,
    Return,

    // Allocation [OpCode <register-count>]
    AllocReg,
    DeallocReg,

    // Library
    PrintAny,
    PrintChar,

    // represents end of instructions
    Halt,
}
