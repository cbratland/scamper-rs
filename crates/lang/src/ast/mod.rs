mod number;
mod value;

pub use number::Number;
pub use value::*;

#[derive(Debug, PartialEq)]
pub struct ListLiteral {
    pub elems: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    String(String),
    Quotation(String),
    Integer(i32),
    Float(f32),
    List(ListLiteral),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub loc: u32,
    pub len: u16,
}

impl Span {
    // beginning of self to end of end
    pub fn to(&self, end: &Span) -> Self {
        Self {
            loc: self.loc,
            len: ((end.loc + end.len as u32) - self.loc) as u16,
        }
    }

    // beginning of self to beginning of end
    pub fn until(&self, end: &Span) -> Self {
        Self {
            loc: self.loc,
            len: (end.loc - self.loc) as u16,
        }
    }

    // end of self to beginning of end
    pub fn between(&self, end: &Span) -> Self {
        Self {
            loc: self.loc + self.len as u32,
            len: (end.loc - self.loc) as u16,
        }
    }

    // get the string in src that the span represents
    pub fn in_src<'a>(&self, src: &'a str) -> &'a str {
        let loc = self.loc as usize;
        &src[loc..loc + self.len as usize]
    }
}

pub type Block = Vec<Operation>;
pub type Label = String;

#[derive(Debug, PartialEq, Clone)]
pub enum OperationKind {
    Variable {
        name: String,
    },
    Value {
        value: Value,
    },
    // lambda
    Closure {
        params: Vec<String>,
        body: Block,
    },
    Application {
        arity: u32,
    },
    If {
        if_block: Block,
        else_block: Block,
    },
    Let {
        names: Vec<String>,
        body: Block,
    },
    // music related
    Sequence {
        subexpr_count: i32,
    },
    Match,
    And {
        jump_to: Label,
    },
    Or {
        jump_to: Label,
    },
    Cond {
        body: Block,
        end: Label,
    },
    Label {
        name: String,
    },
    Exception {
        message: String,
        mod_name: Option<String>,
        span: Option<Span>,
        source: Option<String>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operation {
    pub kind: OperationKind,
    pub span: Span,
}

impl Operation {
    pub fn value(value: Value, span: Span) -> Self {
        Self {
            kind: OperationKind::Value { value },
            span,
        }
    }

    pub fn var(name: String, span: Span) -> Self {
        Self {
            kind: OperationKind::Variable { name },
            span,
        }
    }

    pub fn ap(arity: u32, span: Span) -> Self {
        Self {
            kind: OperationKind::Application { arity },
            span,
        }
    }

    pub fn closure(params: Vec<String>, body: Block, span: Span) -> Self {
        Self {
            kind: OperationKind::Closure { params, body },
            span,
        }
    }

    pub fn let_(names: Vec<String>, body: Block, span: Span) -> Self {
        Self {
            kind: OperationKind::Let { names, body },
            span,
        }
    }

    pub fn if_(if_block: Block, else_block: Block, span: Span) -> Self {
        Self {
            kind: OperationKind::If {
                if_block,
                else_block,
            },
            span,
        }
    }

    pub fn label(name: String) -> Self {
        Self {
            kind: OperationKind::Label { name },
            span: Span { loc: 0, len: 0 },
        }
    }

    pub fn and(jump_to: Label, span: Span) -> Self {
        Self {
            kind: OperationKind::And { jump_to },
            span,
        }
    }

    pub fn or(jump_to: Label, span: Span) -> Self {
        Self {
            kind: OperationKind::Or { jump_to },
            span,
        }
    }

    pub fn cond(body: Block, end: Label, span: Span) -> Self {
        Self {
            kind: OperationKind::Cond { body, end },
            span,
        }
    }

    pub fn exception(
        message: String,
        mod_name: Option<String>,
        span: Option<Span>,
        source: Option<String>,
    ) -> Self {
        Self {
            kind: OperationKind::Exception {
                message,
                mod_name,
                span,
                source,
            },
            span: Span { loc: 0, len: 0 },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StatementKind {
    Binding { name: String, body: Block },
    Expression { body: Block },
    Import { mod_name: String },
    Display { body: Block },
    Struct { id: String, fields: Vec<String> },
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn expr(body: Block, span: Span) -> Self {
        Self::new(StatementKind::Expression { body }, span)
    }

    pub fn binding(name: String, body: Block, span: Span) -> Self {
        Self::new(StatementKind::Binding { name, body }, span)
    }

    pub fn import(mod_name: String, span: Span) -> Self {
        Self::new(StatementKind::Import { mod_name }, span)
    }

    pub fn display(body: Block, span: Span) -> Self {
        Self::new(StatementKind::Display { body }, span)
    }
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub statements: Vec<Statement>,
}
