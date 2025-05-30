#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    None,

    // Load Instructions
    LoadInt16,
    LoadInt32,
    LoadInt64,
    LoadFloat32,
    LoadFloat64,

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
}
