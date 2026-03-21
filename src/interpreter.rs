use crate::lexer::tokenize_line;
use crate::parser::{parse_function, parse_line};
use crate::{
    app_error::AppError,
    types::{Expr, Statement, Value},
};
use std::collections::HashMap;

pub struct Interpreter {
    variables: Vec<HashMap<String, Value>>,
    functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    fn current_scope_mut(&mut self) -> &mut HashMap<String, Value> {
        self.variables.last_mut().unwrap()
    }

    fn current_scope(&self) -> &HashMap<String, Value> {
        self.variables.last().unwrap()
    }

    pub fn execute_source(&mut self, source: String) -> Result<(), AppError> {
        let mut lines = source.lines().peekable();
        let mut statements = Vec::new();

        while let Some(line) = lines.next() {
            let tokens = tokenize_line(line);

            match tokens.as_slice() {
                t if t.len() >= 3 && t[0] == "HOW" && t[1] == "IZ" && t[2] == "I" => {
                    let func = parse_function(&tokens, &mut lines)?;
                    statements.push(func);
                }

                _ => {
                    if let Some(stmt) = parse_line(&tokens)? {
                        statements.push(stmt);
                    }
                }
            }
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
            Statement::Hai(version) => {
                if let Some(version) = version {
                    println!("Using LOLCODE v{version}")
                }
            }
            Statement::Visible(expr) => {
                let value = self.eval_expr(&expr);
                println!("{}", self.value_to_string(&value));
            }
            Statement::IHasA(name, expr) => {
                let value = self.eval_expr(&expr);
                self.current_scope_mut().insert(name.clone(), value);
            }
            Statement::KThxBye => {
                // println!("KTHXBYE")
            }
            Statement::HowIzI(_, _, _) => {
                // functions are already registered.
            }
            Statement::IIz(name, param_values) => {
                let (func_params, func_statements) = self
                    .functions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| AppError::FunctionDoesNotExist(name.clone()))?;

                let arg_values: Vec<Value> = param_values
                    .iter()
                    .map(|expr| self.eval_expr(expr))
                    .collect();

                let mut new_scope = HashMap::new();

                if func_params.len() != arg_values.len() {
                    return Err(AppError::NotEnoughArgsForFunction);
                }

                for (param, value) in func_params.into_iter().zip(arg_values) {
                    new_scope.insert(param, value);
                }

                self.variables.push(new_scope);

                for stmt in &func_statements {
                    self.execute_statement(stmt)?;
                }

                self.variables.pop();
            }
        }
        Ok(())
    }

    fn eval_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Numbar(n) => Value::Numbar(*n),
            Expr::Numbr(n) => Value::Numbr(*n),
            Expr::Yarn(s) => Value::Yarn(s.clone()),
            Expr::Troof(b) => Value::Troof(*b),
            Expr::Variable(name) => self
                .current_scope()
                .get(name)
                .cloned()
                .unwrap_or(Value::Noob),
            Expr::Noob => Value::Noob,
        }
    }

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Numbar(n) => n.to_string(),
            Value::Numbr(n) => n.to_string(),
            Value::Yarn(s) => s.clone(),
            Value::Troof(b) => {
                if *b {
                    "WIN".into()
                } else {
                    "FAIL".into()
                }
            }
            Value::Noob => "NOOB".into(),
        }
    }
}
