#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum KirinType {
    Void,
    Any,
    Null,
    String,
    Int,
    Float,
    Bool,
    Function(Vec<KirinType>, Box<KirinType>),
    Tuple(Vec<KirinType>),
    Struct(String, Vec<KirinType>),
    Vector(Box<KirinType>),
    Array(Box<KirinType>, usize),
}
