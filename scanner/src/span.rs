#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TokenSpan {
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl Default for TokenSpan {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            start: 0,
            end: 0,
        }
    }
}
