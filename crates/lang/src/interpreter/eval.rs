use std::cell::RefCell;
use std::rc::Rc;
use std::vec::IntoIter;

use super::{Env, RuntimeError};
use crate::ast::*;
use crate::parser::keyword::RESERVED_WORDS;

// wasm has a smaller stack size in debug mode
#[cfg(debug_assertions)]
const MAX_CALL_STACK_DEPTH: usize = 25;

#[cfg(not(debug_assertions))]
const MAX_CALL_STACK_DEPTH: usize = 1000;

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Clone)]
pub enum Output {
    Value(Value),
    Error(RuntimeError),
}

pub struct ExecutionStack {
    pub stack: Vec<Value>,
    env: Rc<RefCell<Env>>,
    ops: IntoIter<Operation>,
    op_count: usize,
    current_op: usize,
    call_stack_depth: usize,
}

impl ExecutionStack {
    pub fn new(env: Rc<RefCell<Env>>, body: Block) -> Self {
        let op_count = body.len();
        // if block has only one operation and it is a value, push it to the stack
        if op_count == 1 && matches!(body[0].kind, OperationKind::Value { .. }) {
            let OperationKind::Value { value } = body.into_iter().next().unwrap().kind else {
                panic!("Invalid block")
            };
            Self {
                stack: vec![value],
                env,
                ops: Vec::new().into_iter(),
                op_count: 0,
                current_op: 0,
                call_stack_depth: 0,
            }
        } else {
            Self {
                env,
                stack: Vec::new(),
                ops: body.into_iter(),
                op_count,
                current_op: 0,
                call_stack_depth: 0,
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.current_op >= self.op_count
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.is_done() {
            self.step()?;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}

impl ExecutionStack {
    fn dump_and_switch(
        &mut self,
        new_env: Option<Rc<RefCell<Env>>>,
        new_ops: Vec<Operation>,
        span: Option<Span>,
    ) -> Result<()> {
        self.call_stack_depth += 1;

        if self.call_stack_depth >= MAX_CALL_STACK_DEPTH {
            return Err(RuntimeError::new(
                "Maximum call stack size exceeded".to_string(),
                span,
            ));
        }

        let new_env = new_env.unwrap_or_else(|| self.env.clone());
        let saved_state = (
            std::mem::replace(&mut self.op_count, new_ops.len()),
            std::mem::replace(&mut self.ops, new_ops.into_iter()),
            std::mem::replace(&mut self.current_op, 0),
            std::mem::replace(&mut self.env, new_env),
        );

        let result = self.run();

        self.op_count = saved_state.0;
        self.ops = saved_state.1;
        self.current_op = saved_state.2;
        self.env = saved_state.3;

        self.call_stack_depth -= 1;

        result
    }

    fn jump_to(&mut self, label: String) -> Result<()> {
        self.current_op += 1;
        let mut cur = self.ops.next();

        while let Some(op) = cur {
            match op.kind {
                OperationKind::Label { name } if name == label => {
                    return Ok(());
                }
                _ => {
                    self.current_op += 1;
                    cur = self.ops.next();
                }
            }
        }

        Err(RuntimeError::new(
            format!("Label `{}` not found", label),
            None,
        ))
    }

    fn jump_past(&mut self, label: String) -> Result<()> {
        self.jump_to(label)?;
        self.current_op += 1;
        Ok(())
    }

    fn step(&mut self) -> Result<()> {
        self.current_op += 1;
        let Some(op) = self.ops.next() else {
            return Ok(());
        };
        match op.kind {
            OperationKind::Value { value } => {
                self.stack.push(value);
            }
            OperationKind::Variable { name } => {
                assert_not_reserved(&name)?;
                if let Some(value) = self.env.borrow().get(&name) {
                    self.stack.push(value.clone());
                } else {
                    return Err(RuntimeError::new(
                        format!("Referenced unbound identifier `{name}`"),
                        Some(op.span),
                    ));
                }
            }
            OperationKind::Application { arity } => {
                if self.stack.len() < arity as usize {
                    return Err(RuntimeError::new(
                        format!("Not enough arguments on stack"),
                        Some(op.span),
                    ));
                }
                let args = self.stack.split_off(self.stack.len() - arity as usize);
                let func = self.stack.pop().unwrap();

                match func {
                    Value::Closure(closure) => self.eval_closure(closure, args, op.span)?,
                    Value::Function(function) => {
                        let result = function.0(&args)
                            .map_err(|err| RuntimeError::new(err.message, Some(op.span)))?;
                        self.stack.push(result);
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Non-function value in function application"),
                            Some(op.span),
                        ))
                    }
                }
            }
            OperationKind::Closure { params, body } => {
                let value = Value::Closure(Closure {
                    params,
                    body,
                    env: Some(self.env.clone()),
                });
                self.stack.push(value);
            }
            OperationKind::If {
                if_block,
                else_block,
            } => {
                if self.stack.is_empty() {
                    return Err(RuntimeError::new(
                        format!("Empty stack in if expression"),
                        Some(op.span),
                    ));
                }
                let cond = self.stack.pop().unwrap();
                match cond {
                    Value::Boolean(b) => {
                        if b {
                            self.dump_and_switch(None, if_block, Some(op.span))?;
                        } else {
                            self.dump_and_switch(None, else_block, Some(op.span))?;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            format!("Boolean expected in conditional"),
                            Some(op.span),
                        ));
                    }
                };
            }
            OperationKind::Let { names, body } => {
                for name in &names {
                    assert_not_reserved(&name)?;
                }
                if self.stack.len() < names.len() {
                    return Err(RuntimeError::new(
                        format!("Not enough values on stack for let binding"),
                        Some(op.span),
                    ));
                }

                let values = self.stack.split_off(self.stack.len() - names.len());

                let new_env = self.extend_env(names.into_iter().zip(values.into_iter()));
                self.dump_and_switch(Some(new_env), body, Some(op.span))?;
            }
            OperationKind::Sequence { subexpr_count } => {
                self.stack = self
                    .stack
                    .split_off(self.stack.len() - subexpr_count as usize);

                if self.stack.len() < subexpr_count as usize {
                    return Err(RuntimeError::new(
                        format!(
                            "Not enough values on stack for sequence: {} expected, {} found",
                            subexpr_count,
                            self.stack.len()
                        ),
                        Some(op.span),
                    ));
                }

                let ret = self.stack.pop().unwrap();
                for _ in 1..subexpr_count {
                    self.stack.pop();
                }
                self.stack.push(ret);
            }
            OperationKind::Match { branches } => {
                if self.stack.is_empty() {
                    return Err(RuntimeError::new(
                        format!("Scrutinee missing from stack for match"),
                        Some(op.span),
                    ));
                }
                let scutinee = self.stack.pop().unwrap();
                let mut found_match = false;
                for branch in branches {
                    let bindings = self.try_match(&branch.0, &scutinee)?;
                    if let Some(bindings) = bindings {
                        let new_env = self.extend_env(bindings);
                        self.dump_and_switch(Some(new_env), branch.1, Some(op.span))?;
                        found_match = true;
                        break;
                    }
                }
                if !found_match {
                    return Err(RuntimeError::new(
                        format!("No pattern matches for {}", scutinee),
                        Some(op.span),
                    ));
                }
            }
            OperationKind::And { jump_to } => {
                if self.stack.is_empty() {
                    return Err(RuntimeError::new(
                        format!("Missing argument to \"and\" instruction"),
                        Some(op.span),
                    ));
                }
                let cond = self.stack.pop().unwrap();
                match cond {
                    Value::Boolean(b) => {
                        if !b {
                            self.stack.push(Value::Boolean(false));
                            self.jump_to(jump_to)?;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            format!("\"and\" expects a boolean value"),
                            Some(op.span),
                        ));
                    }
                };
            }
            OperationKind::Or { jump_to } => {
                if self.stack.is_empty() {
                    return Err(RuntimeError::new(
                        format!("Missing argument to \"or\" instruction"),
                        Some(op.span),
                    ));
                }
                let cond = self.stack.pop().unwrap();
                match cond {
                    Value::Boolean(b) => {
                        if b {
                            self.stack.push(Value::Boolean(true));
                            self.jump_to(jump_to)?;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            format!("\"or\" expects a boolean value"),
                            Some(op.span),
                        ));
                    }
                };
            }
            OperationKind::Cond { body, end } => {
                if self.stack.is_empty() {
                    return Err(RuntimeError::new(
                        "missing guard to \"cond\" instruction".to_string(),
                        Some(op.span),
                    ));
                }
                let cond = self.stack.pop().unwrap();
                match cond {
                    Value::Boolean(b) => {
                        if b {
                            self.jump_past(end)?;
                            self.dump_and_switch(None, body, Some(op.span))?;
                        }
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            "boolean expected in conditional".to_string(),
                            Some(op.span),
                        ));
                    }
                };
            }
            OperationKind::Label { .. } => {
                // do nothing
            }
            OperationKind::Exception {
                message,
                mod_name,
                span,
                source,
            } => {
                return Err(RuntimeError::new(
                    format!(
                        "{}{}{}",
                        if let Some(mod_name) = mod_name {
                            format!("[{mod_name}] ")
                        } else {
                            String::default()
                        },
                        if let Some(source) = source {
                            format!("{}: ", source)
                        } else {
                            String::default()
                        },
                        message,
                    ),
                    span,
                ));
            }
        }
        Ok(())
    }

    fn eval_closure(&mut self, closure: Closure, args: Vec<Value>, span: Span) -> Result<()> {
        if closure.params.len() != args.len() {
            return Err(RuntimeError::new(
                format!(
                    "Function expected {} arguments, passed {} instead",
                    closure.params.len(),
                    args.len()
                ),
                Some(span),
            ));
        }

        let new_env = self.extend_env(closure.params.into_iter().zip(args.into_iter()));
        self.dump_and_switch(Some(new_env), closure.body, Some(span))?;

        Ok(())
    }

    // returns None if the pattern does not match the value
    fn try_match(&self, pattern: &Value, value: &Value) -> Result<Option<Vec<(String, Value)>>> {
        let matches = Ok(Some(Vec::new()));
        match (pattern, value) {
            (Value::Symbol(s), v) => {
                if s == "_" {
                    matches
                } else {
                    assert_not_reserved(&s)?;
                    Ok(Some(vec![(s.clone(), v.clone())]))
                }
            }
            (Value::List(p), Value::List(v)) => self.match_lists(p, v),
            (Value::List(p), Value::Pair(vx, vy)) => {
                self.match_lists(p, &vec![*vx.clone(), *vy.clone()])
            }
            (Value::List(p), Value::Struct(v)) => {
                if p.is_empty() {
                    return Ok(None);
                }

                let Value::Symbol(head) = p.first().unwrap() else {
                    return Ok(None);
                };

                if v.kind != *head {
                    return Ok(None);
                }

                let args = p.iter().skip(1).collect::<Vec<&Value>>();

                if v.fields.len() == args.len() {
                    let mut env = Vec::new();
                    for i in 0..v.fields.len() {
                        let env2 = self.try_match(args[i], &v.values[i])?;
                        if env2.is_none() {
                            return Ok(None);
                        }
                        env.extend(env2.unwrap());
                    }
                    Ok(Some(env))
                } else {
                    Ok(None)
                }
            }
            _ => {
                if pattern == value {
                    matches
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn match_lists(&self, p: &Vec<Value>, v: &Vec<Value>) -> Result<Option<Vec<(String, Value)>>> {
        if p.is_empty() || v.is_empty() {
            return Ok(None);
        }

        let head = p.first().unwrap();
        let args = p.iter().skip(1).collect::<Vec<&Value>>();

        if let Value::Symbol(sym) = head {
            if sym == "pair" || sym == "cons" && args.len() == 2 {
                let env1 = self.try_match(args[0], v.first().unwrap())?;
                let v_values = v.iter().skip(1).collect::<Vec<&Value>>();
                let env2 = if v_values.len() == 1 {
                    self.try_match(args[1], v_values.first().unwrap())?
                } else {
                    self.try_match(
                        args[1],
                        &Value::List(
                            v_values
                                .into_iter()
                                .map(|i| i.clone())
                                .collect::<Vec<Value>>(),
                        ),
                    )?
                };
                if let (Some(env1), Some(env2)) = (env1, env2) {
                    let mut env = env1;
                    env.extend(env2);
                    Ok(Some(env))
                } else {
                    Ok(None)
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    fn extend_env(&self, bindings: impl IntoIterator<Item = (String, Value)>) -> Rc<RefCell<Env>> {
        let new_env = Rc::new((*self.env).clone());
        {
            let mut new_env = new_env.borrow_mut();
            for (name, value) in bindings.into_iter() {
                new_env.set(name.clone(), value.clone());
            }
        }
        new_env
    }
}

// Sem
pub struct Runner {
    stmts: IntoIter<Statement>,
    stmt_count: usize,
    output: Vec<Output>,
    env: Rc<RefCell<Env>>,
    current_stmt: usize,
}

impl Runner {
    pub fn new(program: Ast, env: Option<Rc<RefCell<Env>>>) -> Self {
        let stmt_count = program.statements.len();
        let stmts = program.statements.into_iter();
        Self {
            stmts,
            stmt_count,
            output: Vec::new(),
            env: env.unwrap_or(Rc::new(RefCell::new(Env::new(None)))),
            current_stmt: 0,
        }
    }

    pub fn get_output(self) -> Vec<Output> {
        self.output
    }

    pub fn is_done(&self) -> bool {
        self.current_stmt >= self.stmt_count
    }

    pub fn step(&mut self) -> Result<()> {
        let Some(stmt) = self.stmts.next() else {
            return Ok(());
        };
        match stmt.kind {
            StatementKind::Binding { name, body } => {
                self.step_define(name, body)?;
            }
            StatementKind::Expression { body } => {
                self.step_expr(body);
            }
            StatementKind::Import { mod_name } => {
                self.step_import(mod_name)?;
            }
            StatementKind::Display { body } => {
                self.step_expr(body);
            }
            StatementKind::Struct { id, fields } => {
                self.step_struct(id, fields)?;
            }
        }
        self.current_stmt += 1;

        Ok(())
    }

    pub fn execute(&mut self) {
        while !self.is_done() {
            match self.step() {
                Ok(_) => {}
                Err(e) => {
                    self.output.push(Output::Error(e));
                }
            }
        }
    }
}

impl Runner {
    fn step_define(&self, name: String, body: Block) -> Result<()> {
        assert_not_reserved(&name)?;
        let mut interpreter = ExecutionStack::new(Rc::clone(&self.env), body);
        _ = interpreter.run()?;

        let value = interpreter.stack.pop();
        match value {
            Some(value) => {
                // println!("{name} = {}", value);
                self.env.borrow_mut().set(name, value);
            }
            None => {
                todo!();
            }
        }

        Ok(())
    }

    fn step_import(&mut self, mod_name: String) -> Result<()> {
        match mod_name.as_str() {
            "image" => {
                crate::modules::image::add_to(&mut self.env.borrow_mut());
            }
            _ => {
                return Err(RuntimeError::new(
                    format!("Module not found: {}", mod_name),
                    None,
                ));
            }
        };
        Ok(())
    }

    fn step_struct(&mut self, id: String, fields: Vec<String>) -> Result<()> {
        assert_not_reserved(&id)?;
        for field in &fields {
            assert_not_reserved(field)?;
        }

        let s = Struct {
            kind: id,
            fields,
            values: vec![],
        };

        s.add_to(&mut self.env.borrow_mut(), None, None);

        Ok(())
    }

    fn step_expr(&mut self, body: Block) {
        let mut interpreter = ExecutionStack::new(Rc::clone(&self.env), body);
        match interpreter.run() {
            Ok(_) => {
                let value = interpreter.stack.pop();
                match value {
                    Some(value) => {
                        self.output.push(Output::Value(value));
                    }
                    None => {
                        todo!();
                    }
                };
            }
            Err(e) => self.output.push(Output::Error(e)),
        }
    }
}

fn assert_not_reserved(identifier: &str) -> Result<()> {
    if RESERVED_WORDS.contains(&identifier) {
        Err(RuntimeError::new(
            format!(
                "\"{}\" is a reserved word and cannot be used as an identifier",
                identifier
            ),
            None,
        ))
    } else {
        Ok(())
    }
}
