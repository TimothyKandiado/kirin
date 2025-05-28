#[derive(Debug, Clone)]
pub enum ParsedObject {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    Slice(Vec<ParsedObject>),
    Vector(Vec<ParsedObject>),
}
