#[derive(Debug)]
pub enum KirinError {
    General(String),
    Scan(SpannedError),
    Parse(SpannedError),
}

#[derive(Debug)]
pub struct SpannedError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
