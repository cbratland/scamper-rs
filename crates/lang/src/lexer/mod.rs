#[cfg(test)]
mod tests;
mod token;

pub use token::*;

use crate::ast::Span;
use crate::diagnostics::ParseError;
use std::str::Chars;

type Result<T> = std::result::Result<T, ParseError>;

const INVALID_VAR_CHARS: &str = "\",'`()[]{}|;"; // todo: racket says no # in variables. why does scamper allow it?

pub struct Lexer<'a> {
    chars: Chars<'a>,
    pub loc: u32,
    len_remaining: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars(),
            loc: 0,
            len_remaining: src.len(),
        }
    }

    // move to next character
    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    // return next character without consuming
    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    // check if at end of file
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    // update remaining char count
    fn update_len_remaining(&mut self) {
        let new_len = self.chars.as_str().len();
        self.loc += (self.len_remaining - new_len) as u32;
        self.len_remaining = new_len;
    }

    // return number of characters since last update_len_remaining
    fn moved_len(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    fn span(&self) -> Span {
        Span {
            loc: self.loc,
            len: self.moved_len() as u16,
        }
    }

    // eat characters while predicate is true
    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek()) && !self.is_eof() {
            self.next();
        }
    }

    // check if character is a whitespace
    fn is_whitespace(c: char) -> bool {
        matches!(
            c,
            '\u{0009}'   // \t
            | '\u{000A}' // \n
            | '\u{000B}' // vertical tab
            | '\u{000C}' // form feed
            | '\u{000D}' // \r
            | '\u{0020}' // space
            | '\u{0085}'
            // bidi
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK
            // unicode
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR
        )
    }

    // lex the next token
    pub fn next_token(&mut self) -> Result<(Token, /* preceeded by whitespace */ bool)> {
        let mut preceeded_by_whitespace = false;
        loop {
            let first_char = match self.next() {
                Some(c) => c,
                None => {
                    return Ok((
                        Token::new(TokenKind::Eof, self.loc, 0),
                        preceeded_by_whitespace,
                    ))
                }
            };
            let kind = match first_char {
                // comments
                ';' => {
                    self.eat_while(|c| c != '\n');
                    TokenKind::LineComment
                }

                // whitespace
                c if Self::is_whitespace(c) => {
                    preceeded_by_whitespace = true;
                    self.update_len_remaining();
                    continue;
                }

                '(' => TokenKind::OpenDelimiter(Delimiter::Parenthesis),
                ')' => TokenKind::CloseDelimiter(Delimiter::Parenthesis),
                '{' => TokenKind::OpenDelimiter(Delimiter::Brace),
                '}' => TokenKind::CloseDelimiter(Delimiter::Brace),
                '[' => TokenKind::OpenDelimiter(Delimiter::Bracket),
                ']' => TokenKind::CloseDelimiter(Delimiter::Bracket),

                '\'' => TokenKind::Quote,

                '"' => {
                    self.eat_while(|c2| c2 != '\n' && c2 != '"');
                    if self.next().unwrap_or(' ') != '"' {
                        return Err(ParseError::new(
                            "unterminated double quote string",
                            Some(self.span()),
                        )
                        .with_code(1));
                    }
                    TokenKind::Literal(LiteralKind::String)
                }

                c if c.is_numeric() || c == '-' => {
                    if c == '-' && !self.peek().is_numeric() {
                        self.eat_while(|c| {
                            !INVALID_VAR_CHARS.contains(c) && !Self::is_whitespace(c)
                        });
                        TokenKind::Sequence
                    } else {
                        let mut kind = LiteralKind::Integer;
                        loop {
                            match self.peek() {
                                c if c.is_numeric() => (),
                                '.' => kind = LiteralKind::Float,
                                _ => break,
                            };
                            self.next();
                        }
                        TokenKind::Literal(kind)
                    }
                }

                c => {
                    let next = self.peek();

                    let kind = if c == '#' {
                        if ['t', 'f'].contains(&next) {
                            self.next();
                            TokenKind::Literal(LiteralKind::Boolean)
                        } else if next == '\\' {
                            self.eat_while(|c| !Self::is_whitespace(c));
                            TokenKind::Literal(LiteralKind::Char)
                        } else {
                            TokenKind::Sequence
                        }
                    } else {
                        TokenKind::Sequence
                    };

                    if kind == TokenKind::Sequence {
                        self.eat_while(|c| {
                            !INVALID_VAR_CHARS.contains(c) && !Self::is_whitespace(c)
                        });
                    }

                    kind
                }
            };

            let token_len = self.moved_len();
            let loc = self.loc;
            self.update_len_remaining();
            return Ok((
                Token::new(kind, loc, token_len as u16),
                preceeded_by_whitespace,
            ));
        }
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer::new(src);
    let mut tokens: Vec<Token> = vec![];
    loop {
        let token = lexer.next_token()?.0;
        let done = TokenKind::Eof == token.kind;
        tokens.push(token);
        if done {
            break;
        }
    }
    Ok(tokens)
}
