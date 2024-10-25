use crate::ast::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    Parenthesis, // ()
    Brace,       // {}
    Bracket,     // []
}

impl Delimiter {
    // pub fn open(&self) -> char {
    //     match self {
    //         Delimiter::Parenthesis => '(',
    //         Delimiter::Brace => '{',
    //         Delimiter::Bracket => '[',
    //     }
    // }

    pub fn close(&self) -> char {
        match self {
            Delimiter::Parenthesis => ')',
            Delimiter::Brace => '}',
            Delimiter::Bracket => ']',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    String,
    Integer,
    Float,
    Boolean,
    Char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    OpenDelimiter(Delimiter),
    CloseDelimiter(Delimiter),
    LineComment, // ;
    Literal(LiteralKind),
    Quote,    // '
    Sequence, // any other sequence of non-whitespace, non-delimiting characters
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn is_comment(&self) -> bool {
        use TokenKind::*;
        matches!(self.kind, LineComment)
    }

    pub fn is_eof(&self) -> bool {
        use TokenKind::*;
        matches!(self.kind, Eof)
    }

    pub fn as_str<'a>(&self, src: &'a str) -> &'a str {
        let loc = self.span.loc as usize;
        &src[loc..loc + self.span.len as usize]
    }

    // pub fn is_opening_delimiter(&self) -> bool {
    //     use TokenKind::*;
    //     matches!(self.kind, OpenDelimiter(_))
    // }

    // pub fn is_closing_delimiter(&self) -> bool {
    //     use TokenKind::*;
    //     matches!(self.kind, CloseDelimiter(_))
    // }

    // pub fn sequence<'a>(&self, src: &'a str) -> Option<&'a str> {
    //     if self.kind == TokenKind::Sequence {
    //         Some(self.as_str(src))
    //     } else {
    //         None
    //     }
    // }
}

impl Token {
    pub fn new(kind: TokenKind, loc: u32, len: u16) -> Self {
        Self {
            kind,
            span: Span { loc, len },
        }
    }
}
