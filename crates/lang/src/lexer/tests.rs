use super::*;

#[test]
fn empty_string() {
    assert_eq!(
        tokenize("").expect("lex failed"),
        vec![Token::new(TokenKind::Eof, 0, 0)]
    );
}

#[test]
fn define() {
    assert_eq!(
        tokenize("(define a 1)").expect("lex failed"),
        vec![
            Token::new(TokenKind::OpenDelimiter(Delimiter::Parenthesis), 0, 1),
            Token::new(TokenKind::Sequence, 1, 6),
            Token::new(TokenKind::Sequence, 8, 1),
            Token::new(TokenKind::Literal(LiteralKind::Integer), 10, 1),
            Token::new(TokenKind::CloseDelimiter(Delimiter::Parenthesis), 11, 1),
            Token::new(TokenKind::Eof, 12, 0)
        ]
    )
}

#[test]
fn comments() {
    // line comment
    assert_eq!(
        tokenize("; comment\nx").expect("lex failed"),
        vec![
            Token::new(TokenKind::LineComment, 0, 9),
            Token::new(TokenKind::Sequence, 10, 1),
            Token::new(TokenKind::Eof, 11, 0)
        ]
    );
}

#[test]
fn string_char_literals() {
    // empty string
    assert_eq!(
        tokenize("\"\"").expect("lex failed"),
        vec![
            Token::new(TokenKind::Literal(LiteralKind::String), 0, 2),
            Token::new(TokenKind::Eof, 2, 0)
        ]
    );
    // regular string
    assert_eq!(
        tokenize("\"test\"").expect("lex failed"),
        vec![
            Token::new(TokenKind::Literal(LiteralKind::String), 0, 6),
            Token::new(TokenKind::Eof, 6, 0)
        ]
    );
    // unclosed quotes
    assert!(tokenize("\"").is_err());
}

#[test]
fn number_literals() {
    // digit
    assert_eq!(
        tokenize("1").expect("lex failed"),
        vec![
            Token::new(TokenKind::Literal(LiteralKind::Integer), 0, 1),
            Token::new(TokenKind::Eof, 1, 0)
        ]
    );
    // long number (int64 max)
    assert_eq!(
        tokenize("9223372036854775807").expect("lex failed"),
        vec![
            Token::new(TokenKind::Literal(LiteralKind::Integer), 0, 19),
            Token::new(TokenKind::Eof, 19, 0)
        ]
    );
    // float
    assert_eq!(
        tokenize("1.0").expect("lex failed"),
        vec![
            Token::new(TokenKind::Literal(LiteralKind::Float), 0, 3),
            Token::new(TokenKind::Eof, 3, 0)
        ]
    );
}

#[test]
fn quotes() {
    // string
    assert_eq!(
        tokenize("'a").expect("lex failed"),
        vec![
            Token::new(TokenKind::Quote, 0, 1),
            Token::new(TokenKind::Sequence, 1, 1),
            Token::new(TokenKind::Eof, 2, 0)
        ]
    );
    // list
    assert_eq!(
        tokenize("'(a b c)").expect("lex failed"),
        vec![
            Token::new(TokenKind::Quote, 0, 1),
            Token::new(TokenKind::OpenDelimiter(Delimiter::Parenthesis), 1, 1),
            Token::new(TokenKind::Sequence, 2, 1),
            Token::new(TokenKind::Sequence, 4, 1),
            Token::new(TokenKind::Sequence, 6, 1),
            Token::new(TokenKind::CloseDelimiter(Delimiter::Parenthesis), 7, 1),
            Token::new(TokenKind::Eof, 8, 0)
        ]
    );
}
