use super::*;

#[test]
fn empty_source() {
    let src = "";
    let ast = parse(src).expect("parse failed");
    assert!(ast.statements.is_empty());
}

#[test]
fn multi_expr() {
    let src = "(define a 1)\n(+ a 2)\n(+ a 3)";
    let ast = parse(src).expect("parse failed");
    assert_eq!(
        ast,
        Ast {
            statements: vec![
                Statement::binding(
                    String::from("a"),
                    vec![Operation::value(
                        Value::Integer(1),
                        Span { loc: 10, len: 1 }
                    )],
                    Span { loc: 0, len: 12 }
                ),
                Statement::expr(
                    vec![
                        Operation::var(String::from("+"), Span { loc: 14, len: 1 }),
                        Operation::var(String::from("a"), Span { loc: 16, len: 1 }),
                        Operation::value(Value::Integer(2), Span { loc: 18, len: 1 }),
                        Operation::ap(2, Span { loc: 13, len: 7 })
                    ],
                    Span { loc: 13, len: 7 }
                ),
                Statement::expr(
                    vec![
                        Operation::var(String::from("+"), Span { loc: 22, len: 1 }),
                        Operation::var(String::from("a"), Span { loc: 24, len: 1 }),
                        Operation::value(Value::Integer(3), Span { loc: 26, len: 1 }),
                        Operation::ap(2, Span { loc: 21, len: 7 })
                    ],
                    Span { loc: 21, len: 7 }
                )
            ]
        }
    );
}

#[test]
fn variables() {
    let src1 = "a";
    let ast1 = parse(src1).expect("parse failed");
    assert_eq!(
        ast1,
        Ast {
            statements: vec![Statement::expr(
                vec![Operation::var(String::from("a"), Span { loc: 0, len: 1 })],
                Span { loc: 0, len: 1 }
            )]
        }
    );

    let src2 = "(+ 1 a)";
    let ast2 = parse(src2).expect("parse failed");
    assert_eq!(
        ast2,
        Ast {
            statements: vec![Statement::expr(
                vec![
                    Operation::var(String::from("+"), Span { loc: 1, len: 1 }),
                    Operation::value(Value::Integer(1), Span { loc: 3, len: 1 }),
                    Operation::var(String::from("a"), Span { loc: 5, len: 1 }),
                    Operation::ap(2, Span { loc: 0, len: 7 })
                ],
                Span { loc: 0, len: 7 }
            )]
        }
    );
}

#[test]
fn define_stmt() {
    let src1 = "(define a 1)";
    let ast1 = parse(src1).expect("parse failed");
    assert_eq!(
        ast1,
        Ast {
            statements: vec![Statement::binding(
                String::from("a"),
                vec![Operation::value(
                    Value::Integer(1),
                    Span { loc: 10, len: 1 }
                )],
                Span { loc: 0, len: 12 }
            )]
        }
    );

    let src2 = "(define a (+ 1 2))";
    let ast2 = parse(src2).expect("parse failed");
    assert_eq!(
        ast2,
        Ast {
            statements: vec![Statement::binding(
                String::from("a"),
                vec![
                    Operation::var(String::from("+"), Span { loc: 11, len: 1 }),
                    Operation::value(Value::Integer(1), Span { loc: 13, len: 1 }),
                    Operation::value(Value::Integer(2), Span { loc: 15, len: 1 }),
                    Operation::ap(2, Span { loc: 10, len: 7 })
                ],
                Span { loc: 0, len: 18 }
            )]
        }
    );
}

#[test]
fn literals() {
    let int_src = "1";
    let int_ast = parse(int_src).expect("parse failed");
    assert_eq!(
        int_ast.statements,
        vec![Statement::expr(
            vec![Operation {
                kind: OperationKind::Value {
                    value: Value::Integer(1)
                },
                span: Span { loc: 0, len: 1 }
            }],
            Span { loc: 0, len: 1 }
        )]
    );

    let float_src = "1.0";
    let float_ast = parse(float_src).expect("parse failed");
    assert_eq!(
        float_ast.statements,
        vec![Statement::expr(
            vec![Operation {
                kind: OperationKind::Value {
                    value: Value::Float(1.0)
                },
                span: Span { loc: 0, len: 3 }
            }],
            Span { loc: 0, len: 3 }
        )]
    );

    let string_src = "\"hello\"";
    let string_ast = parse(string_src).expect("parse failed");
    assert_eq!(
        string_ast.statements,
        vec![Statement::expr(
            vec![Operation {
                kind: OperationKind::Value {
                    value: Value::String(String::from("hello"))
                },
                span: Span { loc: 0, len: 7 }
            }],
            Span { loc: 0, len: 7 }
        )]
    );
}

#[test]
fn closures() {
    let src = "(define add (lambda (a b) (+ a b)))";
    let ast = parse(src).expect("parse failed");
    assert_eq!(
        ast,
        Ast {
            statements: vec![Statement::binding(
                String::from("add"),
                vec![Operation::closure(
                    vec![String::from("a"), String::from("b")],
                    vec![
                        Operation::var(String::from("+"), Span { loc: 27, len: 1 }),
                        Operation::var(String::from("a"), Span { loc: 29, len: 1 }),
                        Operation::var(String::from("b"), Span { loc: 31, len: 1 }),
                        Operation::ap(2, Span { loc: 26, len: 7 })
                    ],
                    Span { loc: 12, len: 22 }
                )],
                Span { loc: 0, len: 35 }
            )]
        }
    );
}

#[test]
fn if_expr() {
    let src = "(if #t 1 2)";
    let ast = parse(src).expect("parse failed");
    assert_eq!(
        ast,
        Ast {
            statements: vec![Statement::expr(
                vec![
                    Operation::value(Value::Boolean(true), Span { loc: 4, len: 2 }),
                    Operation::if_(
                        vec![Operation::value(Value::Integer(1), Span { loc: 7, len: 1 })],
                        vec![Operation::value(Value::Integer(2), Span { loc: 9, len: 1 })],
                        Span { loc: 0, len: 11 }
                    )
                ],
                Span { loc: 0, len: 11 }
            )]
        }
    );
}
