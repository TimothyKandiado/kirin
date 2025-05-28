#[derive(Debug)]
pub enum KirinError {
    General(String),
    Scan(ScanError),
}

#[derive(Debug)]
pub struct ScanError {
    pub message: String,
    pub line: usize,
}
