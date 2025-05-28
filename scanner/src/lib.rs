pub mod token;

use token::{Token, TokenType};
use errors::{KirinError, ScanError};

fn simple_token(token_type: TokenType, line: usize) -> Token {
    Token {
        token_type,
        lexeme: "".to_string(),
        line,
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
        }
    }

    pub fn scan_tokens_with_filename(self, source: &str, filename: &str) -> Result<TokenContainer, KirinError> {
        let scanned_tokens = self.scan_tokens(source)?;
        let filename = filename.to_string();

        Ok(TokenContainer {
            scanned_tokens,
            filename
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
                let token = simple_token(TokenType::NewLine, self.line);
                tokens.push(token);
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
        });

        Ok(tokens)
    }

    fn generate_error(&self, message: String) -> KirinError {
        KirinError::Scan(
            ScanError {
                message,
                line: self.line,
            }
        )
    }

    fn scan_token(&mut self) -> Result<Token, KirinError> {
        let newline_token = self.skip_whitespace();
        if let Some(newline) = newline_token {
            return Ok(newline);
        }

        self.start = self.current;
        let current_character = self.advance();

        match current_character {
            '+' => Ok(simple_token(TokenType::Plus, self.line)),
            '-' => Ok(simple_token(TokenType::Minus, self.line)),
            '*' => Ok(simple_token(TokenType::Star, self.line)),
            '/' => Ok(simple_token(TokenType::Slash, self.line)),
            '^' => Ok(simple_token(TokenType::Caret, self.line)),
            '%' => Ok(simple_token(TokenType::Percent, self.line)),

            '(' => Ok(simple_token(TokenType::LeftParen, self.line)),
            ')' => Ok(simple_token(TokenType::RightParen, self.line)),
            ':' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::ColonEqual, self.line));
                }

                Ok(simple_token(TokenType::Colon, self.line))
            }
            '.' => Ok(simple_token(TokenType::Dot, self.line)),
            ',' => Ok(simple_token(TokenType::Comma, self.line)),
            '"' => self.scan_string(),
            '&' => {
                let next = self.advance();
                if next == '&' {
                    return Ok(simple_token(TokenType::And, self.line));
                }

                Err(self.generate_error(format!("Unexpected token {}", current_character)))
            }

            '|' => {
                let next = self.advance();
                if next != '|' {
                    return Err(self.generate_error("Unknown token '|' ".to_string()));
                }

                Ok(simple_token(TokenType::Or, self.line))
            }

            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::GreaterEqual, self.line));
                }
                Ok(simple_token(TokenType::Greater, self.line))
            }

            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::LessEqual, self.line));
                }
                Ok(simple_token(TokenType::Less, self.line))
            }

            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::EqualEqual, self.line));
                }
                Ok(simple_token(TokenType::Equal, self.line))
            }

            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(simple_token(TokenType::NotEqual, self.line));
                }

                Ok(simple_token(TokenType::Not, self.line))
            }

            x if x.is_ascii_digit() => self.scan_number(),
            x if is_identifier_start(x) => self.scan_identifier(),

            _ => Err(self.generate_error(format!("Unknown token {}", current_character))),
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
            Some(simple_token(TokenType::NewLine, self.line))
        } else {
            None
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
            line: self.line,
        })
    }

    fn scan_identifier(&mut self) -> Result<Token, KirinError> {
        while !self.is_at_end() && is_identifier_rest(self.peek()) {
            self.advance();
        }

        let segment = &self.source[self.start..self.current];

        match segment {
            "for" => Ok(simple_token(TokenType::For, self.line)),
            "if" => Ok(simple_token(TokenType::If, self.line)),
            "else" => Ok(simple_token(TokenType::Else, self.line)),
            "while" => Ok(simple_token(TokenType::While, self.line)),
            "fn" => Ok(simple_token(TokenType::Fn, self.line)),
            "end" => Ok(simple_token(TokenType::End, self.line)),
            "return" => Ok(simple_token(TokenType::Return, self.line)),
            "true" => Ok(simple_token(TokenType::True, self.line)),
            "false" => Ok(simple_token(TokenType::False, self.line)),
            "and" => Ok(simple_token(TokenType::And, self.line)),
            "or" => Ok(simple_token(TokenType::Or, self.line)),
            "class" => Ok(simple_token(TokenType::Class, self.line)),
            "let" => Ok(simple_token(TokenType::Let, self.line)),
            "block" => Ok(simple_token(TokenType::Block, self.line)),
            "delete" => Ok(simple_token(TokenType::Delete, self.line)),
            "none" => Ok(simple_token(TokenType::None, self.line)),
            "include" => Ok(simple_token(TokenType::Include, self.line)),

            _ => Ok(Token {
                token_type: TokenType::Identifier,
                lexeme: segment.to_string(),
                line: self.line,
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
            line: self.line,
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
    use crate::token::{Token, TokenType};
    use super::{simple_token, Scanner};

    #[test]
    fn test_scanner_number() {
        let source = "100";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Number,
                    lexeme: "100".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1
                }
            ]
        )
    }

    #[test]
    fn test_scanner_identifier() {
        let source = "sin";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Identifier,
                    lexeme: "sin".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1
                }
            ]
        )
    }

    #[test]
    fn test_scanner_keywords() {
        let source = "for while \n fn end";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::For,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::While,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    line: 2
                },
                Token {
                    token_type: TokenType::Fn,
                    lexeme: "".to_string(),
                    line: 2
                },
                Token {
                    token_type: TokenType::End,
                    lexeme: "".to_string(),
                    line: 2
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    line: 2
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 2
                }
            ]
        )
    }

    #[test]
    fn test_scanner_simple_expression() {
        let source = "1 + 2 / ( 3 + 1 )";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token {
                    token_type: TokenType::Number,
                    lexeme: "1".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: "2".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Slash,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::LeftParen,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: "3".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Number,
                    lexeme: "1".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::RightParen,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1
                }
            ]
        )
    }

    #[test]
    fn test_scanner_comparison_operators() {
        let source = "== >= <= > < !=";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                simple_token(TokenType::EqualEqual, 1),
                simple_token(TokenType::GreaterEqual, 1),
                simple_token(TokenType::LessEqual, 1),
                simple_token(TokenType::Greater, 1),
                simple_token(TokenType::Less, 1),
                simple_token(TokenType::NotEqual, 1),
                Token {
                    token_type: TokenType::NewLine,
                    lexeme: "".to_string(),
                    line: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_string(),
                    line: 1
                }
            ]
        )
    }

    #[test]
    fn test_scanner_logical_operators() {
        let source = "&& || ! and or";
        let tokens = Scanner::new().scan_tokens(source).unwrap();

        assert_eq!(
            tokens,
            vec![
                simple_token(TokenType::And, 1),
                simple_token(TokenType::Or, 1),
                simple_token(TokenType::Not, 1),
                simple_token(TokenType::And, 1),
                simple_token(TokenType::Or, 1),
                simple_token(TokenType::NewLine, 1),
                simple_token(TokenType::Eof, 1)
            ]
        )
    }
}
