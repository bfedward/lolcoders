use bigdecimal::ToPrimitive;

use crate::expression::{CastTypes, Expr};
use crate::lexer::{normalise_source, tokenize_line};
use crate::parser::parse_line;

use crate::statement::Statement;
use crate::types::identifier::{Identifier, IdentifierExpr};
use crate::types::primitive::{Numbar, Number, Numbr, Troof, Yarn};
use crate::types::{eval_bool_expr, eval_comparison_expr, eval_maths_expr};
use crate::{app_error::AppError, types::Value};
use std::collections::HashMap;
use std::io::{Write, stdin, stdout};

pub struct Interpreter {
    it_variable: Option<Value>,
    variables: Vec<HashMap<Identifier, Value>>,
    functions: HashMap<Identifier, (Vec<Identifier>, Vec<Statement>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            it_variable: None,
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
            //     Ok(Some(stmt)) => {
            //         println!("Parsed:   {:?}", &stmt);
            //         statements.push(stmt)
            //     },
            //     Ok(None) => (),
            //     Err(e) => {
            //         println!("Error tokens: {}", crate::lexer::Tokens(tokens));
            //         return Err(e);
            //     }
            // }
        }

        if let Some(first) = statements.first()
            && !matches!(first, Statement::Hai(_))
        {
            return Err(AppError::HaiMustBeFirstLine);
        }

        if let Some(last) = statements.last()
            && !matches!(last, Statement::KThxBye)
        {
            return Err(AppError::KThxByeMustBeLastLine);
        }

        for (i, statement) in statements.iter().enumerate() {
            if matches!(statement, Statement::Hai(_)) && i != 0 {
                return Err(AppError::HaiMustBeFirstLine);
            }

            if matches!(statement, Statement::KThxBye) && i != statements.len() - 1 {
                return Err(AppError::KThxByeMustBeLastLine);
            }
        }

        for statement in statements {
            self.execute_statement(&statement)?;
            // match self.execute_statement(&statement) {
            //     Ok(_) => {
            //         println!("Executed: {:?}", statement)
            //     }
            //     Err(e) => {
            //         println!("Error: {:?} ----- {e}", &statement);
            //     }
            // }
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
            Statement::CanHasLib(_) => {
                // this is future lolcode functionality.
            }
            Statement::Visible(exprs, no_new_line) => {
                let expr_values: Vec<Value> = exprs
                    .iter()
                    .map(|expr| self.eval_expr(expr))
                    .collect::<Result<Vec<_>, _>>()?;

                let yarns: Vec<Yarn> = expr_values.iter().map(|y| y.as_yarn()).collect::<Result<
                    Vec<_>,
                    AppError,
                >>(
                )?;

                let concat = yarns.iter().fold(Yarn::new(String::new()), |mut acc, x| {
                    acc.concat(x.clone());
                    acc
                });

                if *no_new_line {
                    print!("{}", concat)
                } else {
                    println!("{}", concat)
                }
            }
            Statement::IHasA(var_name, expr) => {
                let identifier = self.resolve_identifier_expr(var_name)?;

                let value = self.eval_expr(expr)?;

                let curr_scope = self
                    .current_scope_mut()
                    .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                if curr_scope.get(&identifier).is_some() {
                    return Err(AppError::CannotRedeclareVariable(identifier));
                }

                curr_scope.insert(identifier, value);
            }
            Statement::KThxBye => {
                // println!("KTHXBYE")
            }
            Statement::HowIzI(name, params, body) => {
                let id = self.resolve_identifier_expr(name)?;
                let params: Vec<Identifier> = params
                    .iter()
                    .map(|p| self.resolve_identifier_expr(p))
                    .collect::<Result<Vec<Identifier>, AppError>>()?;
                self.functions.insert(id, (params, body.clone()));
            }
            Statement::IIz(func_name, param_values) => {
                let func_name = self.resolve_identifier_expr(func_name)?;
                let (func_params, func_statements) = self
                    .functions
                    .get(&func_name)
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
            Statement::Gimmeh(input_var) => {
                let identifier = self.resolve_identifier_expr(input_var)?;

                let curr_scope_mut = self
                    .current_scope_mut()
                    .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                let _ = curr_scope_mut
                    .get(&identifier)
                    .ok_or(AppError::VariableDoesNotExist(identifier.clone()))?;

                let mut input = String::new();

                stdout().flush().ok();

                stdin()
                    .read_line(&mut input)
                    .map_err(|_| AppError::BadGimmeh)?;

                // the user presses enter at the end of their input, so
                // we need to remove the trailing newline characters
                let input = input.trim_end_matches(&['\n', '\r'][..]).to_string();

                curr_scope_mut
                    .entry(identifier.clone())
                    .and_modify(|e| *e = Value::Yarn(Yarn::new(input)));

                return Ok(());
            }
            Statement::Rassignment(var, expr) => {
                let identifier = self.resolve_identifier_expr(var)?;

                let value = self.eval_expr(expr)?;
                let curr_scope_mut = self
                    .current_scope_mut()
                    .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                let _ = curr_scope_mut
                    .get(&identifier)
                    .ok_or(AppError::VariableDoesNotExist(identifier.clone()))?;

                curr_scope_mut
                    .entry(identifier.clone())
                    .and_modify(|e| *e = value);
            }
            Statement::Recast(id, cast_type) => {
                let identifier = self.resolve_identifier_expr(id)?;

                let curr_scope_mut = self
                    .current_scope_mut()
                    .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                let value = curr_scope_mut
                    .get(&identifier)
                    .ok_or(AppError::VariableDoesNotExist(identifier.clone()))?;

                let casted = if matches!(value, Value::Noob) {
                    match cast_type {
                        CastTypes::Troof => Value::Troof(Troof::default()),
                        CastTypes::Yarn => Value::Yarn(Yarn::default()),
                        CastTypes::Numbr => Value::Numbr(Numbr::default()),
                        CastTypes::Numbar => Value::Numbar(Numbar::default()),
                        CastTypes::Noob => Value::Noob,
                    }
                } else {
                    Self::cast_value(value, cast_type)?
                };

                curr_scope_mut
                    .entry(identifier.clone())
                    .and_modify(|e| *e = casted);
            }
            Statement::VarRIIzFunc(var_name, func_name, param_values) => {
                let var_id = self.resolve_identifier_expr(var_name)?;

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
                                .get(&var_id)
                                .ok_or(AppError::VariableDoesNotExist(var_id.clone()))?;

                            curr_scope_mut
                                .entry(var_id.clone())
                                .and_modify(|e| *e = Value::Noob);

                            return Ok(());
                        }
                        Statement::FoundYr(expr) => {
                            let val = self.eval_expr(expr)?;

                            let previous_scope = self
                                .previous_scope_mut()
                                .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                            let _ = previous_scope
                                .get(&var_id)
                                .ok_or(AppError::VariableDoesNotExist(var_id.clone()))?;

                            previous_scope
                                .entry(var_id.clone())
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
            Statement::Expr(expr) => {
                let val = self.eval_expr(expr)?;
                self.it_variable = Some(val);
            }
            Statement::ORly(o_rly_block) => {
                let it_val = self
                    .it_variable
                    .as_ref()
                    .ok_or(AppError::NoValueInItVariable)?
                    .clone();

                if it_val.as_troof().value() {
                    for stmt in &o_rly_block.ya_rly_block {
                        self.execute_statement(stmt)?;
                    }
                    return Ok(());
                } else {
                    for mebbe_block in &o_rly_block.mebbe_blocks {
                        let expr_val = self.eval_expr(&mebbe_block.expr)?;
                        if expr_val.as_troof().value() {
                            for stmt in &mebbe_block.statements {
                                self.execute_statement(stmt)?;
                            }
                            return Ok(());
                        }
                    }
                }

                for stmt in &o_rly_block.no_wai_block {
                    self.execute_statement(stmt)?;
                }
            }
        }
        Ok(())
    }

    fn resolve_identifier_expr(&self, ident: &IdentifierExpr) -> Result<Identifier, AppError> {
        match ident {
            IdentifierExpr::Identifier(id) => Ok(id.clone()),

            IdentifierExpr::Srs(expr) => {
                let value = self.eval_expr(expr)?;

                let yarn = value.as_yarn()?;

                Ok(Identifier::new(yarn.to_string())?)
            }
        }
    }

    fn interpolate_yarn(&self, input: &Yarn) -> Result<Yarn, AppError> {
        let mut result = String::new();
        let input = input.to_string();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i] == ':' && i + 1 < chars.len() {
                match chars[i + 1] {
                    '{' => {
                        let (interpolated, consumed) =
                            self.interpolate_variable(&chars[i + 2..])?;
                        result.push_str(&interpolated.to_string());
                        i += consumed + 2;
                    }
                    '(' => {
                        let (interpolated, consumed) = self.interpolate_hex(&chars[i + 2..])?;
                        result.push_str(&interpolated.to_string());
                        i += consumed + 2;
                    }
                    '[' => unimplemented!(), //interpolate_char_name(),
                    _ => {
                        result.push(chars[i]);
                        i += 1;
                    }
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        Ok(Yarn::new(result))
    }

    fn interpolate_variable(&self, chars: &[char]) -> Result<(Value, usize), AppError> {
        // find closing brace
        let mut j = 0;
        while j < chars.len() {
            if chars[j] == '}' {
                break;
            } else {
                j += 1;
            }
        }

        // check if j == chars.len()?? What if we have :{ but no }
        if j == chars.len() {
            return Err(AppError::UnclosedInterpolation);
        }

        let inner: String = chars[..j].iter().collect();
        let inner = inner.trim();

        let id = Identifier::new(inner.to_owned())?;

        let curr_scope = self
            .current_scope()
            .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

        let value = curr_scope.get(&id).cloned().unwrap_or(Value::Noob);

        Ok((value, j + 1))
    }

    fn interpolate_hex(&self, chars: &[char]) -> Result<(Value, usize), AppError> {
        // find closing brace
        let mut j = 0;
        while j < chars.len() {
            if chars[j] == ')' {
                break;
            } else {
                j += 1;
            }
        }

        // check if j == chars.len()?? What if we have :{ but no }
        if j == chars.len() {
            return Err(AppError::UnclosedInterpolation);
        }

        let inner: String = chars[..j].iter().collect();
        let inner = inner.trim();

        let codepoint =
            u32::from_str_radix(inner, 16).map_err(|_| AppError::InvalidUnicodeCodepoint)?;
        let ch = char::from_u32(codepoint).ok_or(AppError::InvalidUnicodeCodepoint)?;

        Ok((Value::Yarn(Yarn::new(ch.to_string())), j + 1))
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Value, AppError> {
        match expr {
            Expr::Numbar(n) => Ok(Value::Numbar(n.clone())),
            Expr::Numbr(n) => Ok(Value::Numbr(n.clone())),
            Expr::Yarn(s) => {
                let interpolated_yarn = self.interpolate_yarn(s)?;

                Ok(Value::Yarn(interpolated_yarn))
            }
            Expr::Smoosh(args) => {
                let mut yarn = Yarn::new(String::new());

                for arg in args {
                    let v = self.eval_expr(arg)?.as_yarn()?;
                    yarn.concat(v);
                }

                Ok(Value::Yarn(yarn))
            }
            Expr::Maek(expr, cast_type) => {
                match expr.as_ref() {
                    Expr::Variable(identifier_expr) => {
                        let curr_scope = self
                            .current_scope()
                            .ok_or(AppError::CouldNotGetCurrentVariableScope)?;

                        let identifier = self.resolve_identifier_expr(identifier_expr)?;

                        let value = curr_scope.get(&identifier).cloned().unwrap_or(Value::Noob);

                        let casted = if matches!(value, Value::Noob) {
                            match cast_type {
                                CastTypes::Troof => Value::Troof(Troof::default()),
                                CastTypes::Yarn => Value::Yarn(Yarn::default()),
                                CastTypes::Numbr => Value::Numbr(Numbr::default()),
                                CastTypes::Numbar => Value::Numbar(Numbar::default()),
                                CastTypes::Noob => Value::Noob,
                            }
                        } else {
                            Self::cast_value(&value, cast_type)?
                        };

                        Ok(casted)
                    }
                    _ => {
                        let value = self.eval_expr(expr)?;

                        // now convert the value to whatever the cast_type is.
                        let casted = Self::cast_value(&value, cast_type)?;

                        Ok(casted)
                    }
                }
            }
            Expr::Troof(b) => Ok(Value::Troof(b.clone())),
            Expr::Variable(name) => {
                let name = self.resolve_identifier_expr(name)?;

                Ok(self.lookup_variable(&name).cloned().unwrap_or(Value::Noob))
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
            Expr::Negation(inner) => {
                let inner_value = self.eval_expr(inner)?;
                let mut inner_troof = inner_value.as_troof();
                inner_troof.flip_value();
                Ok(Value::Troof(inner_troof))
            }
        }
    }

    fn cast_value(value: &Value, cast_type: &CastTypes) -> Result<Value, AppError> {
        let casted = match cast_type {
            CastTypes::Troof => Value::Troof(value.as_troof()),
            CastTypes::Yarn => Value::Yarn(value.as_yarn()?),
            CastTypes::Numbr => {
                let number = value.as_number().unwrap_or_default();

                let casted_number = match number {
                    Number::Int(int) => Numbr::new(int),

                    Number::Decimal(decimal) => {
                        let int = decimal.to_i64().ok_or(AppError::NumberOverflow)?;

                        Numbr::new(int)
                    }
                };

                Value::Numbr(casted_number)
            }
            CastTypes::Numbar => {
                let number = value.as_number().unwrap_or_default();

                let casted_number = match number {
                    Number::Int(int) => Numbar::new(int.into()),
                    Number::Decimal(float) => Numbar::new(float),
                };

                Value::Numbar(casted_number)
            }
            CastTypes::Noob => Value::Noob,
        };

        Ok(casted)
    }

    fn lookup_variable(&self, name: &Identifier) -> Option<&Value> {
        self.variables
            .iter()
            .rev() // check the local scope, then the previous scopes.
            .find_map(|scope| scope.get(name))
    }
}
