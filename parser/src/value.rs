#[derive(Debug, Clone)]
pub enum ParsedValue {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Slice(Vec<ParsedValue>),
    Vector(Vec<ParsedValue>),
}
