use crate::span::TokenSpan;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TokenType {
    Number,
    String,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Identifier,
    Dot,
    Comma,
    None,
    Eof,
    Caret,

    Fn,
    Class,
    Let,
    Block,
    Delete,
    If,
    Else,
    For,
    While,
    NewLine,
    End,
    Return,
    True,
    False,
    Include,

    And,
    Or,
    Not,
    Equal,
    EqualEqual,
    ColonEqual,
    NotEqual,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub span: TokenSpan,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.token_type {
                TokenType::Number | TokenType::String | TokenType::Identifier => {
                    format!("({:?} : {})", self.token_type, self.lexeme)
                }

                _ => {
                    format!("{:?}", self.token_type)
                }
            }
        )
    }
}

pub fn debug_print_tokens(tokens: Vec<Token>) {
    for token in tokens {
        print!("{} ", &token);
        if TokenType::NewLine == token.token_type {
            println!();
        }
    }

    println!();
}
