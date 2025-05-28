pub mod token;
mod span;

use errors::{KirinError, ScanError};
use token::{Token, TokenType};
use crate::span::TokenSpan;

fn simple_token(token_type: TokenType, span: TokenSpan) -> Token {
    Token {
        token_type,
        lexeme: "".to_string(),
        span,
    }
}

pub struct TokenContainer {
    pub scanned_tokens: Vec<Token>,
    pub filename: String,
}

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,

    line: usize,
    column: usize,
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens_with_filename(
        self,
        source: &str,
        filename: &str,
    ) -> Result<TokenContainer, KirinError> {
        let scanned_tokens = self.scan_tokens(source)?;
        let filename = filename.to_string();

        Ok(TokenContainer {
            scanned_tokens,
            filename,
        })
    }

    pub fn scan_tokens(mut self, source: &str) -> Result<Vec<Token>, KirinError> {
        self.source = source.to_string();
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.scan_token()?;
            tokens.push(token);
        }

        if let Some(last) = tokens.last() {
            if last.token_type != TokenType::NewLine {
                let token = simple_token(TokenType::NewLine, self.get_span());
                tokens.push(token);
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            span: self.get_span(),
        });

        Ok(tokens)
    }

    fn generate_error(&self, message: String) -> KirinError {
        KirinError::Scan(ScanError {
            message,
            line: self.line,
        })
    }

    fn scan_token(&mut self) -> Result<Token, KirinError> {
        let newline_token = self.skip_whitespace();
        if let Some(newline) = newline_token {
            return Ok(newline);
        }

        self.start = self.current;
        let current_character = self.advance();

        match current_character {
            '+' => Ok(simple_token(TokenType::Plus, self.get_span())),
            '-' => Ok(simple_token(TokenType::Minus, self.get_span())),
            '*' => Ok(simple_token(TokenType::Star, self.get_span())),
            '/' => Ok(simple_token(TokenType::Slash, self.get_span())),
            '^' => Ok(simple_token(TokenType::Caret, self.get_span())),
            '%' => Ok(simple_token(TokenType::Percent, self.get_span())),
            '(' => Ok(simple_token(TokenType::LeftParen, self.get_span())),
            ')' => Ok(simple_token(TokenType::RightParen, self.get_span())),
            '{' => Ok(simple_token(TokenType::LeftBrace, self.get_span())),
            '}' => Ok(simple_token(TokenType::RightBrace, self.get_span())),
            '[' => Ok(simple_token(TokenType::LeftBracket, self.get_span())),
            ']' => Ok(simple_token(TokenType::RightBracket, self.get_span())),
            ':' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::ColonEqual, self.get_span()));
                }

                Ok(simple_token(TokenType::Colon, self.get_span()))
            }
            '.' => Ok(simple_token(TokenType::Dot, self.get_span())),
            ',' => Ok(simple_token(TokenType::Comma, self.get_span())),
            '"' => self.scan_string(),
            '&' => {
                let next = self.advance();
                if next == '&' {
                    return Ok(simple_token(TokenType::And, self.get_span()));
                }

                Err(self.generate_error(format!("Unexpected character {}", current_character)))
            }

            '|' => {
                let next = self.advance();
                if next != '|' {
                    return Err(self.generate_error("Unknown character '|' ".to_string()));
                }

                Ok(simple_token(TokenType::Or, self.get_span()))
            }

            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::GreaterEqual, self.get_span()));
                }
                Ok(simple_token(TokenType::Greater, self.get_span()))
            }

            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::LessEqual, self.get_span()));
                }
                Ok(simple_token(TokenType::Less, self.get_span()))
            }

            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::EqualEqual, self.get_span()));
                }
                Ok(simple_token(TokenType::Equal, self.get_span()))
            }

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::NotEqual, self.get_span()));
                }

                Ok(simple_token(TokenType::Not, self.get_span()))
            }

            x if x.is_ascii_digit() => self.scan_number(),
            x if is_identifier_start(x) => self.scan_identifier(),

            _ => Err(self.generate_error(format!("Unknown character {}", current_character))),
        }
    }

    fn skip_whitespace(&mut self) -> Option<Token> {
        let mut has_consumed_newline = false;

        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' => {
                    self.advance();
                }

                '\n' => {
                    has_consumed_newline = true;
                    self.line += 1;
                    self.column = 0;
                    self.advance();
                }

                '#' => {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    self.line += 1;
                    self.advance();
                }

                _ => break,
            }
        }

        if has_consumed_newline {
            self.column = 0;
            Some(simple_token(TokenType::NewLine, self.get_span()))
        } else {
            None
        }
    }

    fn get_span(&self) -> TokenSpan {
        TokenSpan {
            line: self.line,
            column: self.column,
            start: self.start,
            end: self.current
        }
    }

    fn emit_token(&self, token_type: TokenType, lexeme: String) -> Token {
        Token {
            token_type,
            lexeme,
            span: self.get_span(),
        }
    }

    fn scan_number(&mut self) -> Result<Token, KirinError> {
        // consume all digits until the end or non digit character
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        let next = self.peek();
        // if next character is a decimal point consume all remaining digits
        if next == '.' {
            self.advance();
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let next = self.peek();

        // scan exponent section if any
        if next == 'E' {
            self.advance();

            if self.peek() == '-' {
                self.advance();
            }
            self.start = self.current;

            while !self.is_at_end() && self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let segment = self.source[self.start..self.current].to_string();

        Ok(Token {
            token_type: TokenType::Number,
            lexeme: segment,
            span: self.get_span(),
        })
    }

    fn scan_identifier(&mut self) -> Result<Token, KirinError> {
        while !self.is_at_end() && is_identifier_rest(self.peek()) {
            self.advance();
        }

        let segment = &self.source[self.start..self.current];

        match segment {
            "for" => Ok(simple_token(TokenType::For, self.get_span())),
            "if" => Ok(simple_token(TokenType::If, self.get_span())),
            "else" => Ok(simple_token(TokenType::Else, self.get_span())),
            "while" => Ok(simple_token(TokenType::While, self.get_span())),
            "fn" => Ok(simple_token(TokenType::Fn, self.get_span())),
            "end" => Ok(simple_token(TokenType::End, self.get_span())),
            "return" => Ok(simple_token(TokenType::Return, self.get_span())),
            "true" => Ok(simple_token(TokenType::True, self.get_span())),
            "false" => Ok(simple_token(TokenType::False, self.get_span())),
            "and" => Ok(simple_token(TokenType::And, self.get_span())),
            "or" => Ok(simple_token(TokenType::Or, self.get_span())),
            "class" => Ok(simple_token(TokenType::Class, self.get_span())),
            "let" => Ok(simple_token(TokenType::Let, self.get_span())),
            "block" => Ok(simple_token(TokenType::Block, self.get_span())),
            "delete" => Ok(simple_token(TokenType::Delete, self.get_span())),
            "none" => Ok(simple_token(TokenType::None, self.get_span())),
            "include" => Ok(simple_token(TokenType::Include, self.get_span())),

            _ => Ok(Token {
                token_type: TokenType::Identifier,
                lexeme: segment.to_string(),
                span: self.get_span(),
            }),
        }
    }

    fn scan_string(&mut self) -> Result<Token, KirinError> {
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }
        self.consume('"', "Expect \" at end of string")?;

        let mut string = self.source[self.start..self.current].to_string();
        string.remove(0);
        string.remove(string.len() - 1);

        Ok(Token {
            token_type: TokenType::String,
            lexeme: string,
            span: self.get_span(),
        })
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.current += 1;
        self.column += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn consume(&mut self, character: char, message: &str) -> Result<(), KirinError> {
        if self.peek() == character {
            self.advance();
            return Ok(());
        }

        Err(self.generate_error(message.to_string()))
    }
}

fn is_identifier_start(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}

fn is_identifier_rest(character: char) -> bool {
    is_identifier_start(character) || character.is_ascii_digit()
}
#[cfg(test)]
mod scanner_tests {
    use crate::span::TokenSpan;
    use super::{Scanner, simple_token};
    use crate::token::{Token, TokenType};

    fn assert_scanned_tokens(left: Vec<Token>, right: Vec<Token>) {
        let mapped_left = left.iter().map(|v| {
            return (v.token_type, &v.lexeme);
        }).collect::<Vec<(TokenType, &String)>>();

        let mapped_right = right.iter().map(|v| {
            return (v.token_type, &v.lexeme);
        }).collect::<Vec<(TokenType, &String)>>();

        assert_eq!(mapped_left, mapped_right);
    }

    #[test]
    fn test_scanner_number() {
        let source = "100";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Number,
                    lexeme: "100".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                }
            ]
        )
    }

    #[test]
    fn test_scanner_identifier() {
        let source = "sin";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Identifier,
                    lexeme: "sin".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                }
            ]
        )
    }

    #[test]
    fn test_scanner_keywords() {
        let source = "for while \n fn end";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                Token {
                    token_type: TokenType::For,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::While,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Fn,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::End,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                }
            ]
        )
    }

    #[test]
    fn test_scanner_simple_expression() {
        let source = "1 + 2 / ( 3 + 1 )";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Number,
                    lexeme: "1".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: "2".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Slash,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::LeftParen,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: "3".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: "1".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::RightParen,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                }
            ]
        )
    }

    #[test]
    fn test_scanner_comparison_operators() {
        let source = "== >= <= > < !=";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                simple_token(TokenType::EqualEqual, TokenSpan::default()),
                simple_token(TokenType::GreaterEqual, TokenSpan::default()),
                simple_token(TokenType::LessEqual, TokenSpan::default()),
                simple_token(TokenType::Greater, TokenSpan::default()),
                simple_token(TokenType::Less, TokenSpan::default()),
                simple_token(TokenType::NotEqual, TokenSpan::default()),
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    span: TokenSpan::default()
                }
            ]
        )
    }

    #[test]
    fn test_scanner_logical_operators() {
        let source = "&& || ! and or";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                simple_token(TokenType::And, TokenSpan::default()),
                simple_token(TokenType::Or, TokenSpan::default()),
                simple_token(TokenType::Not, TokenSpan::default()),
                simple_token(TokenType::And, TokenSpan::default()),
                simple_token(TokenType::Or, TokenSpan::default()),
                simple_token(TokenType::NewLine, TokenSpan::default()),
                simple_token(TokenType::Eof, TokenSpan::default())
            ]
        )
    }

    #[test]
    fn test_scanner_brackets() {
        let source = "[1, 2]";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_scanned_tokens(
            tokens,
            vec![
                simple_token(TokenType::LeftBracket, TokenSpan::default()),
                Token {
                    token_type: TokenType::Number,
                    lexeme: "1".to_string(),
                    span: TokenSpan::default()
                },
                simple_token(TokenType::Comma, TokenSpan::default()),
                Token {
                    token_type: TokenType::Number,
                    lexeme: "2".to_string(),
                    span: TokenSpan::default()
                },
                simple_token(TokenType::RightBracket, TokenSpan::default()),
                simple_token(TokenType::NewLine, TokenSpan::default()),
                simple_token(TokenType::Eof, TokenSpan::default())
            ]
        )
    }
}
