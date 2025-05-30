use instructions::OpCode;

// compile time u8 representations of OpCodes
pub const OP_NONE: u8 = OpCode::None as u8;

// Load Instructions
pub const OP_LOAD_CONST: u8 = OpCode::LoadConst as u8;
pub const OP_LOAD_INT16: u8 = OpCode::LoadInt16 as u8;

// Mathematical Instructions [OpCode dest src1 src2]
pub const OP_ADD_INT: u8 = OpCode::AddInt as u8;
pub const OP_ADD_FLOAT: u8 = OpCode::AddFloat as u8;
pub const OP_SUB_INT: u8 = OpCode::SubInt as u8;
pub const OP_SUB_FLOAT: u8 = OpCode::SubFloat as u8;
pub const OP_MUL_INT: u8 = OpCode::MulInt as u8;
pub const OP_MUL_FLOAT: u8 = OpCode::MulFloat as u8;
pub const OP_DIV_INT: u8 = OpCode::DivInt as u8;
pub const OP_DIV_FLOAT: u8 = OpCode::DivFloat as u8;
pub const OP_MOD_INT: u8 = OpCode::ModInt as u8;
pub const OP_MOD_FLOAT: u8 = OpCode::ModFloat as u8;
pub const OP_POW_INT: u8 = OpCode::PowInt as u8;
pub const OP_POW_FLOAT: u8 = OpCode::PowFloat as u8;

// Type Casting
pub const OP_INT_TO_ANY: u8 = OpCode::IntToAny as u8;
pub const OP_FLOAT_TO_ANY: u8 = OpCode::FloatToAny as u8;

// Frames
pub const OP_INIT_FRAME: u8 = OpCode::InitFrame as u8;
pub const OP_DROP_FRAME: u8 = OpCode::DropFrame as u8;
pub const OP_RETURN: u8 = OpCode::Return as u8;

// Allocation
pub const OP_ALLOC_REG: u8 = OpCode::AllocReg as u8;
pub const OP_DEALLOC_REG: u8 = OpCode::DeallocReg as u8;

// Library
pub const OP_PRINT_ANY: u8 = OpCode::PrintAny as u8;
pub const OP_PRINT_CHAR: u8 = OpCode::PrintChar as u8;

// Represents end of instructions
pub const OP_HALT: u8 = OpCode::Halt as u8;
