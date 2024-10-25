use std::cell::RefCell;
use std::rc::Rc;
use std::vec::IntoIter;

use super::{Env, RuntimeError};
use crate::ast::*;

type Result<T> = std::result::Result<T, RuntimeError>;

pub struct ExecutionStack {
    pub stack: Vec<Value>,
    env: Rc<RefCell<Env>>,
    ops: IntoIter<Operation>,
    op_count: usize,
    current_op: usize,
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
            }
        } else {
            Self {
                env,
                stack: Vec::new(),
                ops: body.into_iter(),
                op_count,
                current_op: 0,
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
}

impl ExecutionStack {
    fn dump_and_switch(
        &mut self,
        new_env: Option<Rc<RefCell<Env>>>,
        new_ops: Vec<Operation>,
    ) -> Result<()> {
        let curr_count = std::mem::replace(&mut self.op_count, new_ops.len());
        let curr_ops = std::mem::replace(&mut self.ops, new_ops.into_iter());
        let curr_current = std::mem::replace(&mut self.current_op, 0);
        let new_env = new_env.unwrap_or(self.env.clone());
        let curr_env = std::mem::replace(&mut self.env, new_env);

        while !self.is_done() {
            self.step()?;
        }
        self.ops = curr_ops;
        self.op_count = curr_count;
        self.current_op = curr_current;
        self.env = curr_env;

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
                // todo: assert the name is not reserved
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
                    Value::Closure { params, body } => {
                        if params.len() != args.len() {
                            return Err(RuntimeError::new(
                                format!(
                                    "Function expected {} arguments, passed {} instead",
                                    params.len(),
                                    args.len()
                                ),
                                Some(op.span),
                            ));
                        }

                        let new_env = Rc::new((*self.env).clone());
                        {
                            let mut new_env = new_env.borrow_mut();
                            for (key, value) in params.iter().zip(args.iter()) {
                                new_env.set(key.clone(), value.clone());
                            }
                        }

                        self.dump_and_switch(Some(new_env), body)?;
                    }
                    Value::Function(function) => {
                        let result = function.0(&args)?;
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
                let value = Value::Closure { params, body };
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
                            self.dump_and_switch(None, if_block)?;
                        } else {
                            self.dump_and_switch(None, else_block)?;
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
            _ => todo!(),
        }
        Ok(())
    }
}

// Sem
pub struct Runner {
    stmts: IntoIter<Statement>,
    stmt_count: usize,
    output: Vec<Value>,
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

    pub fn get_output(self) -> Vec<Value> {
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
                self.step_expr(body)?;
            }
            StatementKind::Import { mod_name } => {
                self.step_import(mod_name)?;
            }
            StatementKind::Display { body } => {
                // todo: do something different?
                self.step_expr(body)?;
            }
            StatementKind::Struct { id, fields } => {
                self.step_struct(id, fields)?;
            }
        }
        self.current_stmt += 1;

        Ok(())
    }

    pub fn execute(&mut self) -> Result<()> {
        while !self.is_done() {
            self.step()?;
        }
        Ok(())
    }
}

impl Runner {
    fn step_define(&self, name: String, body: Block) -> Result<()> {
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

    fn step_struct(&mut self, _id: String, _fields: Vec<String>) -> Result<()> {
        todo!();
    }

    fn step_expr(&mut self, body: Block) -> Result<()> {
        let mut interpreter = ExecutionStack::new(Rc::clone(&self.env), body);
        _ = interpreter.run()?;
        let value = interpreter.stack.pop();
        match value {
            Some(value) => {
                self.output.push(value);
            }
            None => {
                todo!();
            }
        };
        Ok(())
    }
}
