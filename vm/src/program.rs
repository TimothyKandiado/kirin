use instructions::Instruction;
use std::fs::Metadata;

#[derive(Debug, Clone)]
pub struct Program {
    metadata: ProgramMetadata,
    pub instructions: Vec<Instruction>,
    pub constants: Vec<ProgramConstant>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<ProgramConstant>) -> Self {
        let version = env!("CARGO_PKG_VERSION");

        let mut version_parts = version.split('.');

        let major = version_parts
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);
        let minor = version_parts
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(0);

        let metadata = ProgramMetadata {
            instruction_count: instructions.len(),
            constant_count: constants.len(),
            version_major: major,
            version_minor: minor,
        };

        Self {
            metadata,
            instructions,
            constants,
        }
    }

    pub fn metadata(&self) -> ProgramMetadata {
        self.metadata
    }
}

#[derive(Debug, Clone)]
pub enum ProgramConstant {
    Int32(i32),
    Int64(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Copy, Clone)]
pub struct ProgramMetadata {
    pub version_major: usize,
    pub version_minor: usize,
    pub instruction_count: usize,
    pub constant_count: usize,
}
