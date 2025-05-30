use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum KirinError {
    General(String),
    Scan(SpannedError),
    Parse(SpannedError),
    Runtime(SpannedError),
}

impl Display for KirinError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::General(error) => write!(f, "[Error]: {}", error),
            Self::Scan(error) => write!(
                f,
                "[Scan Error] [line: {}, column: {}]: {}\n",
                error.line, error.column, error.message,
            ),
            Self::Parse(error) => write!(
                f,
                "[Parse Error] [line: {}, column: {}]: {}\n",
                error.line, error.column, error.message,
            ),
            Self::Runtime(error) => write!(
                f,
                "[Runtime Error] [line: {}, column: {}]: {}\n",
                error.line, error.column, error.message,
            ),
        }
    }
}

#[derive(Debug)]
pub struct SpannedError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
