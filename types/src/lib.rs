#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum KirinType {
    Void,
    Unknown,
    Any,
    Null,
    Str,
    Int,
    Float,
    Bool,
    Function(Vec<KirinType>, Box<KirinType>),
    Tuple(Vec<KirinType>),
    Struct(String, Vec<KirinType>),
}
