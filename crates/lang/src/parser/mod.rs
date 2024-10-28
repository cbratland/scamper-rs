#[allow(non_upper_case_globals)]
pub mod keyword;

#[cfg(test)]
pub mod tests;

use crate::ast::*;
use crate::diagnostics::ParseError;
use crate::lexer::{LiteralKind, Token, TokenKind};

type Result<T> = std::result::Result<T, ParseError>;

const NAMED_CHAR_VALUES: [(&str, char); 9] = [
    ("alarm", '\x07'),
    ("backspace", '\x08'),
    ("delete", '\x7F'),
    ("escape", '\x1B'),
    ("newline", '\n'),
    ("null", '\0'),
    ("return", '\r'),
    ("space", ' '),
    ("tab", '\t'),
];

const SPECIAL_FORMS: [&str; 11] = [
    keyword::Lambda,
    keyword::Let,
    keyword::LetStar,
    keyword::And,
    keyword::Or,
    keyword::If,
    keyword::Begin,
    keyword::Match,
    keyword::Cond,
    keyword::Quote,
    keyword::Section,
];

#[derive(Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    loc: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, loc: 0 }
    }

    pub fn next(&mut self) -> Token {
        let next = self.tokens.get(self.loc);
        self.loc += 1;
        match next {
            Some(token) => {
                if token.is_comment() {
                    self.next()
                } else {
                    token.clone()
                }
            }
            None => Token::new(TokenKind::Eof, 0, 0),
        }
    }
}

pub struct Parser<'a> {
    pub src: &'a str,
    pub token: Token, // current token
    pub prev_token: Token,
    pub stream: TokenStream,
    pub label_counter: u32,
}

impl<'a> Parser<'a> {
    // create parser
    pub fn new(src: &'a str, tokens: Vec<Token>) -> Self {
        let mut stream = TokenStream::new(tokens);
        Self {
            src,
            token: stream.next(),
            prev_token: Token::new(TokenKind::Eof, 0, 0),
            stream,
            label_counter: 0,
        }
    }

    pub fn new_raw(src: &'a str) -> Result<Self> {
        let tokens = crate::lexer::tokenize(src)?;
        let mut stream = TokenStream::new(tokens);
        Ok(Self {
            src,
            token: stream.next(),
            prev_token: Token::new(TokenKind::Eof, 0, 0),
            stream,
            label_counter: 0,
        })
    }

    // move to the next token
    pub fn next(&mut self) {
        let next = self.stream.next();
        self.prev_token = std::mem::replace(&mut self.token, next);
        if self.token.is_comment() {
            self.next();
        }
    }

    // check if current token is kind
    pub fn check(&self, token: &TokenKind) -> bool {
        self.token.kind == *token
    }

    // consume current token if it's kind, otherwise return false
    pub fn eat(&mut self, token: &TokenKind) -> bool {
        let present = self.check(token);
        if present {
            self.next();
        }
        present
    }

    // check if current token is keyword
    // pub fn check_keyword(&self, keyword: &str) -> bool {
    //     self.token.kind == TokenKind::Sequence && self.token.as_str(self.src) == keyword
    // }

    // consume current token if it's specified keyword
    // pub fn eat_keyword(&mut self, keyword: &str) -> bool {
    //     let present = self.check_keyword(keyword);
    //     if present {
    //         self.next();
    //     }
    //     present
    // }

    // get token n spaces ahead
    // pub fn peek(&self, n: usize) -> Token {
    //     let mut stream = self.stream.clone();
    //     let mut token = Token::new(TokenKind::Eof, 0, 0);
    //     let mut i = 0;
    //     while i < n {
    //         token = stream.next();
    //         i += 1;
    //     }
    //     token
    // }

    // eat a token and error if it fails
    // pub fn expect(&mut self, token: &TokenKind) -> Result<()> {
    //     if self.eat(token) {
    //         Ok(())
    //     } else {
    //         Err(ParseError::expected_token(token, self.token.span))
    //     }
    // }

    // eat a keyword and error if it fails
    // pub fn expect_keyword(&mut self, keyword: &str) -> Result<()> {
    //     if self.eat_keyword(keyword) {
    //         Ok(())
    //     } else {
    //         Err(ParseError::expected_keyword(keyword, self.token.span))
    //     }
    // }

    pub fn fresh_label(&mut self) -> String {
        let label = format!("lbl_{}", self.label_counter);
        self.label_counter += 1;
        label
    }
}

#[derive(Debug, Clone)]
pub enum ParserValueKind {
    Value(Value),
    Symbol(String),
    List(Vec<ParserValue>),
    Vector(Vec<ParserValue>),
}

#[derive(Debug, Clone)]
pub struct ParserValue {
    pub kind: ParserValueKind,
    pub span: Span,
}

impl ParserValue {
    pub fn sym(sym: String, span: Span) -> Self {
        Self {
            kind: ParserValueKind::Symbol(sym),
            span,
        }
    }

    pub fn list(values: Vec<ParserValue>, span: Span) -> Self {
        Self {
            kind: ParserValueKind::List(values),
            span,
        }
    }
}

impl Into<Value> for ParserValue {
    fn into(self) -> Value {
        match self.kind {
            ParserValueKind::Value(value) => value,
            ParserValueKind::Symbol(sym) => Value::Symbol(sym),
            ParserValueKind::List(values) => {
                let values: Vec<Value> = values.into_iter().map(|v| v.into()).collect();
                Value::List(values)
            }
            ParserValueKind::Vector(values) => {
                let values: Vec<Value> = values.into_iter().map(|v| v.into()).collect();
                Value::Vector(values)
            }
        }
    }
}

// value parsing
impl<'a> Parser<'a> {
    pub fn parse_value(&mut self) -> Result<ParserValue> {
        if let TokenKind::OpenDelimiter(open_delimiter) = self.token.kind {
            let begin = self.token.span;
            self.next();

            let mut values: Vec<ParserValue> = Vec::new();

            while !matches!(
                self.token.kind,
                TokenKind::CloseDelimiter(_) | TokenKind::Eof
            ) {
                values.push(self.parse_value()?);
            }

            let end = self.token.span;

            if self.token.is_eof() {
                // NOTE: error is localized to the open bracket. We could go the end of file here, instead.
                return Err(ParseError::new("unclosed delimiter", Some(begin)));
            } else if let TokenKind::OpenDelimiter(close_delimiter) = self.token.kind {
                if open_delimiter != close_delimiter {
                    return Err(ParseError::new(
                        format!(
                            "mismatched closing delimiter: `{}`",
                            close_delimiter.close()
                        ),
                        Some(end),
                    ));
                }
            }

            self.next();

            if open_delimiter.is_bracket() {
                // treat as list
                Ok(ParserValue {
                    kind: ParserValueKind::Vector(values),
                    span: begin.to(&end),
                })
            } else {
                Ok(ParserValue {
                    kind: ParserValueKind::List(values),
                    span: begin.to(&end),
                })
            }
        } else if self.check(&TokenKind::Quote) {
            // treat as (quote whatever)
            todo!()
        } else {
            // parse single
            self.parse_single(true)
        }
    }

    pub fn parse_single(&mut self, wild_allowed: bool) -> Result<ParserValue> {
        let str = self.token.as_str(&self.src);
        let kind = match self.token.kind {
            TokenKind::Literal(literal) => match literal {
                LiteralKind::Float => {
                    ParserValueKind::Value(Value::Float(str.parse::<f64>().expect("invalid float")))
                }
                LiteralKind::Integer => ParserValueKind::Value(Value::Integer(
                    str.parse::<i64>().expect("invalid integer"),
                )),
                LiteralKind::String => {
                    ParserValueKind::Value(Value::String(str.trim_matches('"').to_string()))
                }
                LiteralKind::Boolean => ParserValueKind::Value(Value::Boolean(str == "#t")),
                LiteralKind::Char => {
                    let c = str.trim_start_matches("#\\");
                    if c.len() == 1 {
                        ParserValueKind::Value(Value::Char(c.chars().next().unwrap()))
                    } else if let Some(char) = NAMED_CHAR_VALUES
                        .iter()
                        .find(|&&(key, _)| key == c)
                        .map(|&(_, value)| value)
                    {
                        ParserValueKind::Value(Value::Char(char))
                    } else {
                        return Err(ParseError::new(
                            "invalid character literal",
                            Some(self.token.span),
                        ));
                    }
                }
            },
            _ => match str {
                "null" => ParserValueKind::Value(Value::Null),
                _ => {
                    if !wild_allowed && str.starts_with('_') {
                        return Err(ParseError::new("identifiers cannot begin with `_` unless inside of `section` or patterns", Some(self.token.span)));
                    }
                    ParserValueKind::Symbol(str.to_string())
                }
            },
        };
        self.next();
        Ok(ParserValue {
            kind,
            span: self.prev_token.span,
        })
    }
}

// statement parsing
impl<'a> Parser<'a> {
    // parse all statements in the token stream
    pub fn parse_statements(&mut self) -> Result<Vec<Statement>> {
        let mut stmts = vec![];

        while let Some(stmt) = self.parse_stmt()? {
            stmts.push(stmt);
        }

        if !self.eat(&TokenKind::Eof) {
            // todo: when does this happen?
            panic!("expected EOF")
        } else {
            Ok(stmts)
        }
    }

    pub fn parse_stmt(&mut self) -> Result<Option<Statement>> {
        if self.token.is_eof() {
            return Ok(None);
        }

        let value = self.parse_value()?;

        if let ParserValueKind::List(ref values) = value.kind {
            if values.is_empty() {
                return Ok(Some(Statement::expr(
                    vec![Operation::value(Value::Null, value.span)],
                    value.span,
                )));
            }

            let head = &values[0];
            let args = &values[1..];

            if let ParserValueKind::Symbol(ref sym) = head.kind {
                match sym.as_str() {
                    keyword::Define => {
                        if args.len() != 2 {
                            return Err(ParseError::new(
                                "define statements must have 2 sub-components: an identifier and a body",
                                Some(value.span),
                            ));
                        }

                        let name = match &args[0].kind {
                            ParserValueKind::Symbol(sym) => sym.clone(),
                            _ => return Err(ParseError::new(
                                "the first component of a define statement must be an identifier",
                                Some(args[0].span),
                            )),
                        };
                        let body = match value.kind {
                            ParserValueKind::List(values) => self.lower(
                                values
                                    .into_iter()
                                    .nth(2)
                                    .expect("define statement must have a body"),
                            )?,
                            _ => todo!(),
                        };
                        return Ok(Some(Statement::binding(name, body, value.span)));
                    }
                    keyword::Import => {
                        if args.len() != 1 {
                            return Err(ParseError::new(
                                "import statements must have 1 argument: the name of a module",
                                Some(value.span),
                            ));
                        }

                        let name = match &args[0].kind {
                            ParserValueKind::Symbol(sym) => sym.clone(),
                            _ => {
                                return Err(ParseError::new(
                                    "the argument of an import statement must be a module name",
                                    Some(args[0].span),
                                ))
                            }
                        };
                        return Ok(Some(Statement::import(name, value.span)));
                    }
                    keyword::Display => {
                        if args.len() != 1 {
                            return Err(ParseError::new("display statements must have 1 argument, the expression to display", Some(value.span)));
                        }
                        let span = value.span;
                        // todo: is this right?
                        return Ok(Some(Statement::display(self.lower(value)?, span)));
                    }
                    keyword::Struct => todo!(),
                    _ => {}
                }
            }
        }

        let span = value.span;
        Ok(Some(Statement::expr(self.lower(value)?, span)))
    }

    pub fn lower(&mut self, value: ParserValue) -> Result<Vec<Operation>> {
        match value.kind {
            ParserValueKind::Value(val) => Ok(vec![Operation::value(val, value.span)]),
            ParserValueKind::Symbol(sym) => Ok(vec![Operation::var(sym, value.span)]),
            ParserValueKind::Vector(vec) => Ok(vec![Operation::value(
                Value::Vector(vec.into_iter().map(|v| v.into()).collect()),
                value.span,
            )]),

            ParserValueKind::List(values) => {
                if values.is_empty() {
                    return Ok(vec![Operation::value(Value::Null, value.span)]);
                }

                let arity = values.len() as u32 - 1;
                let head = &values[0];

                // handle special forms (if, let, etc.)
                if let ParserValueKind::Symbol(sym) = &head.kind {
                    let form = sym.as_ref();
                    if SPECIAL_FORMS.contains(&form) {
                        let args = &values[1..];
                        return self.handle_special_form(form, args, value.span);
                    }
                }

                let mut ops = vec![];
                for value in values {
                    ops.extend(self.lower(value)?);
                }
                ops.push(Operation {
                    kind: OperationKind::Application { arity },
                    span: value.span,
                });
                Ok(ops)
            }
        }
    }
}

impl<'a> Parser<'a> {
    pub fn handle_special_form(
        &mut self,
        form: &str,
        args: &[ParserValue],
        span: Span,
    ) -> Result<Vec<Operation>> {
        match form {
            keyword::Lambda => self.parse_lambda(args, span),
            keyword::Let => self.parse_let(args, span),
            keyword::LetStar => self.parse_let_star(args, span),
            keyword::And => self.parse_and(args, span),
            keyword::Or => self.parse_or(args, span),
            keyword::If => self.parse_if(args, span),
            keyword::Match => self.parse_match(args, span),
            keyword::Cond => self.parse_cond(args, span),
            _ => todo!(),
        }
    }

    pub fn parse_lambda(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        if args.len() != 2 {
            return Err(ParseError::new(
                "lambda expression must have 2 sub-components: a parameter list and a body",
                Some(span),
            ));
        }

        let ParserValueKind::List(lmbda_args) = &args[0].kind else {
            return Err(ParseError::new(
                "the first component of a lambda expression must be a parameter list",
                Some(args[0].span),
            ));
        };

        let mut params = vec![];
        for arg in lmbda_args {
            if let ParserValueKind::Symbol(sym) = &arg.kind {
                params.push(sym.clone());
            } else {
                return Err(ParseError::new(
                    "parameters must only be identifiers",
                    Some(arg.span),
                ));
            }
        }

        Ok(vec![Operation::closure(
            params,
            self.lower(args[1].clone())?,
            span,
        )])
    }

    pub fn parse_let(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        if args.len() != 2 {
            return Err(ParseError::new(
                "let expression must have 2 sub-components: a binding list and a body",
                Some(self.token.span),
            ));
        }

        let ParserValueKind::List(bindings) = &args[0].kind else {
            return Err(ParseError::new(
                "let expression bindings must be given as a list",
                Some(args[0].span),
            ));
        };

        let parsed_bindings = bindings
            .iter()
            .map(|binding| self.parse_binding(binding))
            .collect::<Result<Vec<_>>>()?;

        let names = parsed_bindings
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();

        let mut ops = parsed_bindings
            .into_iter()
            .flat_map(|(_, ops)| ops)
            .collect::<Vec<_>>();

        ops.push(Operation::let_(names, self.lower(args[1].clone())?, span));

        Ok(ops)
    }

    fn parse_let_star(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        if args.len() != 2 {
            return Err(ParseError::new(
                "let* expression must have 2 sub-components: a binding list and a body",
                Some(self.token.span),
            ));
        }

        let ParserValueKind::List(bindings) = &args[0].kind else {
            return Err(ParseError::new(
                "let* expression bindings must be given as a list",
                Some(args[0].span),
            ));
        };

        if bindings.is_empty() {
            return Err(ParseError::new(
                "binding pair must be given as a vector",
                Some(args[0].span),
            ));
        };

        let mut value = ParserValue::list(
            vec![
                ParserValue::sym(String::from("let"), span),
                ParserValue::list(vec![bindings.last().unwrap().clone()], span),
                args[1].clone(),
            ],
            span,
        );
        for binding in bindings.iter().rev().skip(1) {
            value = ParserValue::list(
                vec![
                    ParserValue::sym(String::from("let"), span),
                    ParserValue::list(vec![binding.clone()], span),
                    value,
                ],
                span,
            );
        }

        self.lower(value)
    }

    fn parse_binding(&mut self, value: &ParserValue) -> Result<(String, Vec<Operation>)> {
        let ParserValueKind::Vector(binding) = &value.kind else {
            return Err(ParseError::new(
                "binding pair must be given as a vector",
                Some(value.span),
            ));
        };

        if binding.len() != 2 {
            return Err(ParseError::new(
                "binding must be a pair of a name and value",
                Some(value.span),
            ));
        }

        let name = match &binding[0].kind {
            ParserValueKind::Symbol(sym) => sym.clone(),
            _ => {
                return Err(ParseError::new(
                    "the first component of a binding must be a symbol",
                    Some(value.span),
                ))
            }
        };

        Ok((name, self.lower(binding[1].clone())?))
    }

    pub fn parse_and(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        let label = self.fresh_label();
        Ok(args
            .iter()
            .map(|arg| self.lower(arg.clone()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|ops| {
                let mut ops = ops;
                ops.push(Operation::and(label.clone(), span));
                ops
            })
            .chain(vec![
                Operation::value(Value::Boolean(true), span),
                Operation::label(label.clone()),
            ])
            .collect())
    }

    pub fn parse_or(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        let label = self.fresh_label();
        Ok(args
            .iter()
            .map(|arg| self.lower(arg.clone()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|ops| {
                let mut ops = ops;
                ops.push(Operation::or(label.clone(), span));
                ops
            })
            .chain(vec![
                Operation::value(Value::Boolean(false), span),
                Operation::label(label.clone()),
            ])
            .collect())
    }

    pub fn parse_if(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        if args.len() != 3 {
            return Err(ParseError::new(
                "if expression must have 3 sub-expressions: a guard, if-branch, and else-branch",
                Some(self.token.span),
            ));
        }

        let mut ops = self.lower(args[0].clone())?;
        ops.push(Operation::if_(
            self.lower(args[1].clone())?,
            self.lower(args[2].clone())?,
            span,
        ));
        Ok(ops)
    }

    pub fn parse_match(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        if args.len() < 2 {
            return Err(ParseError::new(
				"match expression must have at least two sub-expressions: a scrutinee at least one branch",
				Some(self.token.span),
			));
        }

        let scrutinee = self.lower(args[0].clone())?;
        let branches = args
            .iter()
            .skip(1)
            .map(|v| self.parse_match_branch(v))
            .collect::<Result<Vec<_>>>()?;

        let mut ops = scrutinee;
        ops.push(Operation::match_(branches, span));
        Ok(ops)
    }

    fn parse_match_branch(&mut self, value: &ParserValue) -> Result<MatchBranch> {
        let ParserValueKind::Vector(branch) = &value.kind else {
            return Err(ParseError::new(
                "match branch must be given as a vector",
                Some(value.span),
            ));
        };
        if branch.len() != 2 {
            return Err(ParseError::new(
                "match branches must be given as a pair of a pattern and an expression",
                Some(value.span),
            ));
        }
        Ok((branch[0].clone().into(), self.lower(branch[1].clone())?))
    }

    pub fn parse_cond(&mut self, args: &[ParserValue], span: Span) -> Result<Vec<Operation>> {
        if args.len() < 1 {
            return Err(ParseError::new(
                "cond expression must have at least one branch",
                Some(span),
            ));
        }

        let label = self.fresh_label();

        Ok(args
            .iter()
            .map(|v| self.parse_cond_branch(v))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flat_map(|(cond, body)| {
                let mut ops = cond;
                ops.push(Operation::cond(body, label.clone(), span));
                ops
            })
            .chain(vec![
                Operation::exception(
                    "No branches of \"cond\" expression matched".into(),
                    None,
                    Some(span),
                    None,
                ),
                Operation::label(label.clone()),
            ])
            .collect())
    }

    // returns the condition and body operations
    fn parse_cond_branch(
        &mut self,
        value: &ParserValue,
    ) -> Result<(Vec<Operation>, Vec<Operation>)> {
        let ParserValueKind::Vector(branch) = &value.kind else {
            return Err(ParseError::new(
                "cond branch must be given as a vector",
                Some(value.span),
            ));
        };

        if branch.len() != 2 {
            return Err(ParseError::new(
                "cond branch must be a pair of expressions",
                Some(value.span),
            ));
        }

        Ok((
            self.lower(branch[0].clone())?,
            self.lower(branch[1].clone())?,
        ))
    }
}

// parses a list of tokens into an ast struct
pub fn parse(src: &str) -> Result<Ast> {
    let mut parser = Parser::new_raw(src)?;
    let statements = parser.parse_statements()?;
    Ok(Ast { statements })
}
