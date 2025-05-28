use scanner::TokenSpan;

#[derive(Clone, Debug)]
pub struct AstSpan {
    pub line: usize,
    pub column: usize,
    pub filename: Option<String>,
}

impl AstSpan {
    pub fn from_token_span(span: TokenSpan, filename: Option<String>) -> AstSpan {
        let TokenSpan {
            line,
            column,
            start: _start,
            end: _end,
        } = span;

        Self {
            line,
            column,
            filename,
        }
    }
}
