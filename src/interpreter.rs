use crate::expression::Expr;
use crate::lexer::{normalise_source, tokenize_line};
use crate::parser::parse_line;

use crate::types::identifier::Identifier;
use crate::types::{eval_bool_expr, eval_comparison_expr, eval_maths_expr};
use crate::{
    app_error::AppError,
    types::{Statement, Value},
};
use std::collections::HashMap;

pub struct Interpreter {
    // it_variable: Option<Value>,
    variables: Vec<HashMap<Identifier, Value>>,
    functions: HashMap<Identifier, (Vec<Identifier>, Vec<Statement>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            // it_variable: None,
            variables: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    fn current_scope_mut(&mut self) -> Option<&mut HashMap<Identifier, Value>> {
        self.variables.last_mut()
    }

    fn current_scope(&self) -> Option<&HashMap<Identifier, Value>> {
        self.variables.last()
    }

    // This is for values returned from functions.
    // Functions run in the variable scope stacked above the calling scope,
    // so the returned value is for a variable in the calling scope.
    fn previous_scope_mut(&mut self) -> Option<&mut HashMap<Identifier, Value>> {
        if self.variables.len() < 2 {
            return None;
        }

        let idx = self.variables.len() - 2;
        self.variables.get_mut(idx)
    }

    pub fn execute_source(&mut self, source: String) -> Result<(), AppError> {
        let normalised_source = normalise_source(source)?;
        let mut lines = normalised_source.lines().peekable();
        let mut statements = Vec::new();

        while let Some(line) = lines.next() {
            let tokens = tokenize_line(line)?;

            if let Some(stmt) = parse_line(&tokens, &mut lines)? {
                statements.push(stmt);
            }

            // // keep this for debugging for now
            // match parse_line(&tokens, &mut lines) {
            //     Ok(Some(stmt)) => statements.push(stmt),
            //     Ok(None) => (),
            //     Err(e) => {
            //         println!("{}", Tokens(tokens));
            //         return Err(e);
            //     }
            // }
        }

        for (i, statement) in statements.iter().enumerate() {
            if matches!(statement, Statement::Hai(_)) && i != 0 {
                return Err(AppError::HaiMustBeFirstLine);
            }

            if matches!(statement, Statement::KThxBye) && i != statements.len() - 1 {
                return Err(AppError::KThxByeMustBeLastLine);
            }
        }

        // register all the functions so they can be called before their declaration
        for stmt in &statements {
            if let Statement::HowIzI(name, params, body) = stmt {
                self.functions
                    .insert(name.clone(), (params.clone(), body.clone()));
            }
        }

        for statement in statements {
            self.execute_statement(&statement)?;
        }

        Ok(())
    }

    fn execute_statement(&mut self, stmt: &Statement) -> Result<(), AppError> {
        match stmt {
            Statement::Hai(_version) => {
                // if let Some(version) = version {
                //     println!("Using LOLCODE v{version}")
                // }
            }
            Statement::Visible(expr) => {
                let value = self.eval_expr(expr)?;
                println!("{}", value);
            }
            Statement::IHasA(var_name, expr) => {
                let value = self.eval_expr(expr)?;

                let curr_scope = self
                    .current_scope_mut()
                    .ok_or(AppError::CouldNotGetCurrentVariableScope)?;
                curr_scope.insert(var_name.clone(), value);
            }
            Statement::KThxBye => {
                // println!("KTHXBYE")
            }
            Statement::HowIzI(_, _, _) => {
                // functions are already registered.
            }
            Statement::IIz(func_name, param_values) => {
                let (func_params, func_statements) = self
                    .functions
                    .get(func_name)
                    .cloned()
                    .ok_or_else(|| AppError::FunctionDoesNotExist(func_name.clone()))?;

                let arg_values: Vec<Value> = param_values
                    .iter()
                    .map(|expr| self.eval_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;

                let mut new_scope = HashMap::new();

                if func_params.len() != arg_values.len() {
                    return Err(AppError::NotEnoughArgsForFunction);
                }

                for (param, value) in func_params.into_iter().zip(arg_values) {
                    new_scope.insert(param, value);
                }

                self.variables.push(new_scope);

                for stmt in &func_statements {
                    match stmt {
                        Statement::Gtfo => {
                            self.variables.pop();
                            return Ok(());
                        }
                        Statement::FoundYr(_) => {
                            self.variables.pop();
                            return Ok(());
                        }
                        _ => self.execute_statement(stmt)?,
                    }
                }

                self.variables.pop();
            }
            Statement::FoundYr(_) | Statement::Gtfo => {
                return Err(AppError::CannotReturnFromFunctionOutsideFunction);
            }
            Statement::VarRIIzFunc(var_name, func_name, param_values) => {
                let (func_params, func_statements) = self
                    .functions
                    .get(func_name)
                    .cloned()
                    .ok_or_else(|| AppError::FunctionDoesNotExist(func_name.clone()))?;

                let arg_values: Vec<Value> = param_values
                    .iter()
                    .map(|expr| self.eval_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;

                let mut new_scope = HashMap::new();

                if func_params.len() != arg_values.len() {
                    return Err(AppError::NotEnoughArgsForFunction);
                }

                for (param, value) in func_params.into_iter().zip(arg_values) {
                    new_scope.insert(param, value);
                }

                self.variables.push(new_scope);

                for stmt in &func_statements {
                    match stmt {
                        Statement::Gtfo => {
                            self.variables.pop();

                            let curr_scope_mut = self
                                .current_scope_mut()
                                .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                            let _ = curr_scope_mut
                                .get(var_name)
                                .ok_or(AppError::VariableDoesNotExist(var_name.clone()))?;

                            curr_scope_mut
                                .entry(var_name.clone())
                                .and_modify(|e| *e = Value::Noob);

                            return Ok(());
                        }
                        Statement::FoundYr(expr) => {
                            let val = self.eval_expr(expr)?;

                            let previous_scope = self
                                .previous_scope_mut()
                                .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                            let _ = previous_scope
                                .get(var_name)
                                .ok_or(AppError::VariableDoesNotExist(var_name.clone()))?;

                            previous_scope
                                .entry(var_name.clone())
                                .and_modify(|e| *e = val);

                            // we're exiting a function so pop the current scope
                            self.variables.pop();

                            return Ok(());
                        }
                        _ => self.execute_statement(stmt)?,
                    }
                }

                self.variables.pop();
            }
        }
        Ok(())
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Value, AppError> {
        match expr {
            Expr::Numbar(n) => Ok(Value::Numbar(n.clone())),
            Expr::Numbr(n) => Ok(Value::Numbr(n.clone())),
            Expr::Yarn(s) => Ok(Value::Yarn(s.clone())),
            Expr::Troof(b) => Ok(Value::Troof(b.clone())),
            Expr::Variable(name) => {
                let curr_scope = self
                    .current_scope()
                    .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                Ok(curr_scope.get(name).cloned().unwrap_or(Value::Noob))
            }
            Expr::Noob => Ok(Value::Noob),
            Expr::Math(math_expr) => {
                let left = self.eval_expr(&math_expr.left)?;
                let right = self.eval_expr(&math_expr.right)?;

                eval_maths_expr(math_expr, left, right)
            }
            Expr::Bool { op, args } => {
                let values: Vec<Value> = args
                    .iter()
                    .map(|expr| self.eval_expr(expr))
                    .collect::<Result<Vec<Value>, AppError>>()?;

                eval_bool_expr(op, values)
            }
            Expr::Comparison(comparison_expr) => {
                let left = self.eval_expr(&comparison_expr.left)?;
                let right = self.eval_expr(&comparison_expr.right)?;

                eval_comparison_expr(comparison_expr, left, right)
            }
        }
    }
}
