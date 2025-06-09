#[repr(u8)]
#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum KirinType {
    Void,
    Any,
    Null,
    String,
    Int,
    Float,
    Bool,
    Variable,
}

impl KirinType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            x if x == KirinType::Void as u8 => Some(Self::Void),
            x if x == KirinType::Any as u8 => Some(Self::Any),
            x if x == KirinType::Null as u8 => Some(Self::Null),
            x if x == KirinType::String as u8 => Some(Self::String),
            x if x == KirinType::Int as u8 => Some(Self::Int),
            x if x == KirinType::Float as u8 => Some(Self::Float),
            x if x == KirinType::Bool as u8 => Some(Self::Bool),
            _ => None,
        }
    }
}
