pub mod Enums;
pub mod Stack;

use Enums::*;
use Stack::*;

#[warn(dead_code)]
pub struct Variable {
    name:             String,
    value:            String,
    var_parsed_type:  TokenT,
    declared_type:    TokenT,
    
}

impl Variable { 
    pub fn new(nm: String, v: String, dt: TokenT, pt: TokenT) -> Self {
        Self {
            name: nm,
            value: v,
            var_parsed_type: pt,
            declared_type: dt,
        }
    }
    
    pub fn empty() -> Self {
        Self {
            name: "".to_string(),
            value: "".to_string(),
            var_parsed_type: TokenT::NONE__,
            declared_type: TokenT::NONE__,
        }
    }
    
    pub fn parse_as_i32(&mut self) -> i32 {
        not_implemented("Variable::parse_as_i32()");
        0
    }

    pub fn is_number(&mut self) -> bool {
        not_implemented("Variable::is_number()");
        false
    }
}


pub struct KasperParser<'a> {
    pub lexer: KasperLexer<'a>,
    stack: Stack,
}

impl<'a> KasperParser<'a> {
    
     pub fn new(lex: KasperLexer<'a>) -> Self {
        Self {
            lexer: lex,
            stack: Stack::new(),
        };
    }
    
    pub fn get_str_var(&mut self, key: String) Option<&String> {
        if self.str_map.contains_key(&key) {
            return Some(&self.str_map[&key]);
        }
        
        return None
    }

    pub fn get_int_var(&mut self, key: String, type_: TokenT) Option<&String>{
        
        match type_ {
            TokenT::INT_T => {
                if self.int_map.contains_key(&key) {
                   return Some(&self.int_map[&key]); // 32
                }
            },
            TokenT::INT_T_64 => {
                if self.int_map_64.contains_key(&key) {
                   return Some(&self.int_map_64[&key]); // I64
                }
            }
        }
        
        return None
    }

    pub fn parse_lexer(&mut self) -> Result<(), io::Error> {
        
        let token = match_lexer_token(self.lexer.next());        
        if token.token_type != TokenT::COMMENT__ && token.token_type != TokenT::NONE__ {        

            if token.token_type == TokenT::FUNC_CALL__ {
                if token.value == String::from(PRINT) {
                    return self.parse_print();
                }

                let err = format!("{}:{}:{} {} is not defined", self.lexer.file_path, token.loc.col, token.loc.row, token.value);

                return Err(make_error(&err));
            }

            if token.token_type == TokenT::KEY_WORD_DEFINE__ {
                return self.parse_def();
            }

           return Ok(());

        }
        return Ok(());
    
    }    
    
    pub fn defined(&mut self, key: &String) -> bool {
        return (
               self.int_map.contains_key(key) 
            || self.str_map.contains_key(key) 
            || self.int_map_64.contains_key(key)
        );
    }
    
    pub fn parse_arithmatic(&mut self, dst_key: &String) -> Result<(), io::Error> {
        if self.defined(dst_key) {
        }
        
        let err = format!("{}:{} Undefined variable {}", self.lexer.row, self.lexer.col, dst_key.to_string());
        return Err(make_error(&err));
    }

    pub fn parse_lhs(&mut self) -> Result<Variable, io::Error> {
        // Parses the type of the variable and makes room for it.
        let mut var   = Variable::empty();
        let mut token = match_lexer_token(self.lexer.next()); 
        if token.token_type == TokenT::VARNAME__ {
             
            var.name = token.value;
            token = match_lexer_token(self.lexer.next()); // ->
            if token.token_type == TokenT::THIN_ARROW__ {
                token = match_lexer_token(self.lexer.next()); // ->

                if token.token_type == TokenT::STRING_T {
                    var.declared_type = TokenT::STRING_T;
                    self.str_map.insert(var.name.clone(), "".to_string());
                    return Ok(var);
                }

                if token.token_type == TokenT::INT_T {
                    var.declared_type = TokenT::INT_T;
                    self.int_map.insert(var.name.clone(), 0);
                    return Ok(var);
                }

                if token.token_type == TokenT::INT_T_64 {
                    var.declared_type = TokenT::INT_T_64;
                    self.int_map_64.insert(var.name.clone(), 0);
                    return Ok(var);

                }

                let err = format!("{}:{} unsupported type {}", token.loc.row, token.loc.col, token.value);
                return Err(make_error(&err));
            }

            let err = format!("{}:{} expected -> but found {}", token.loc.row, token.loc.col, token.value);
            return Err(make_error(&err));
 
        }

        let err = format!("{}:{} expected variable name, found {} instead.", token.loc.row, token.loc.col, token.token_type);
        return Err(make_error(&err));
     
    }
    
    pub fn add(&mut self, variable_name: String, number: i32) {
        if let Some(var) = self.int_map.get_mut(&variable_name) {
            *var += number;
        }
    }
    
    pub fn sub(&mut self, variable_name: String, number: i32) {
        
        if let Some(var) = self.int_map.get_mut(&variable_nam) {
            *var -= number;
        }    
 
    }
    
    pub fn multiply(&mut self, variable_name: String, number: i32) {
        
        if let Some(var) = self.int_map.get_mut(&variable_name) {
            *var *= number;
        }

    }
    
    pub fn divide(&mut self, variable_name: String, number: i32) {
        if let Some(var) = self.int_map.get_mut(&variable_name) {
            *var /= number;
        }
    }
    
    pub fn bit_shift_left(&mut self, variable_name: String, bits: i32) {
        if let Some(var) = self.int_map.get_mut(&variable_name) {
            *var = (*var << bits);
        }
    }

    pub fn bit_shift_right(&mut self, variable_name: String, bits: i32) {
        if let Some(var) = self.int_map.get_mut(&variable_name) {
            *var = (*var >> bits);
        }
    }

    pub fn register_var(&mut self, variable: Variable) -> Result<(), io::Error>{
        
        let mut token = match_lexer_token(self.lexer.next());
        // Declared int so we are expecting a number.
        if variable.declared_type == TokenT::INT_T && token.token_type == TokenT::NUMBER__ {
            let v = token.value.parse::<i32>().unwrap();
            if let Some(var) = self.int_map.get_mut(&variable.name) {
                *var = v;
            }
            
            token = match_lexer_token(self.lexer.next());
            
            match token {
                TokenT::PLUS__  => {
                    token = match_lexer_token(self.lexer.next());
                    match token.token_type {
                        TokenT::NUMBER__ => {
                            let v = token.value.parse::<i32>().unwrap();
                            self.add(variable.name, v);
                        },
                        _ => {
                            let err = format!("{}:{} expected token type {} but got {}", token.loc.row, token.loc.col, TokenT::NUMBER__, token.token_type);
                            return Err(make_error(&err));
                        }
                    }
                },
                TokenT::MINUS__ => {
                    token = match_lexer_token(self.lexer.next());
                    match token.token_type {
                        TokenT::NUMBER__ => {
                            let v = token.value.parse::<i32>().unwrap();
                            self.sub(variable.name, v);
                        },
                         _ => {
                            let err = format!("{}:{} expected token type {} but got {}", token.loc.row, token.loc.col, TokenT::NUMBER__, token.token_type);
                            return Err(make_error(&err));
                        }
                    }            
                },
                TokenT::MULT__  => {
                    token = match_lexer_token(self.lexer.next());
                    match token.token_type {
                        TokenT::NUMBER__ => {
                            let v = token.value.parse::<i32>().unwrap();
                            self.multiply(variable.name, v);
                        },
                        _ => {
                            let err = format!("{}:{} expected token type {} but got {}", token.loc.row, token.loc.col, TokenT::NUMBER__, token.token_type);
                            return Err(make_error(&err));
                        }
                    }                
                }, 
            }

            return Ok(());
        }

        if variable.declared_type == TokenT::STRING_T && token.token_type == TokenT::STRING__ {
            
            if let Some(var) = self.str_map.get_mut(&variable.name) {
                *var = token.value;
            }

            return Ok(());
        }
        
        if variable.declared_type == TokenT::INT_T_64 && token.token_type == TokenT::NUMBER__ {
            
            let v = token.value.parse::<i64>().unwrap();            
            if let Some(var) = self.int_map_64.get_mut(&variable.name) {
                *var = v;
            }

            return Ok(());
        }
        
        let err = format!("{}:{} expected value of type {} but found type {}", token.loc.row, token.loc.col, variable.declared_type, token.token_type);
        return Err(make_error(&err));
    }

    pub fn parse_def(&mut self) -> Result<(), io::Error>{
        match self.parse_lhs() {
            Ok(variable) => {
                // successs parsing the type..
                if match_lexer_token(self.lexer.next()).token_type == TokenT::EQUAL__ {
                    // Assign variable. = val.
                    self.lexer.chop();
                    return self.register_var(variable);
                }
                return Ok(());
            },

            Err(e) => return Err(e),
            _ => panic!("unreachable!"),
        } 
    }
       

    pub fn parse_print(&mut self) -> Result<(), io::Error> {
        let token = match_lexer_token(self.lexer.next());       
        if token.token_type == TokenT::STRING__ {
            
            // Get the print value.
            let val = token.value.clone();    
            
            if match_lexer_token(self.lexer.next()).token_type == TokenT::CPAR__ {
                print!("{}", val);
                return Ok(());
            }
            
            // Error
            let mut err_text = format!("{}:{}:{} unclosed parent found {} expected )..\n", self.lexer.file_path, token.loc.row, token.loc.col, token.value);
            err_text    += &format!("maybe you meant: print?.");
            return Err(make_error(&err_text));
            
        }
        
        if token.token_type == TokenT::VARNAME__ {
            let k = &token.value;
            
            if self.int_map.contains_key(k) {
                let v = &self.int_map[k];
                println!("{}", v);
                return Ok(());
            }

            if self.str_map.contains_key(k) {
                let v = &self.str_map[k];
                println!("{}", v);
                return Ok(());
            }
            
            if self.int_map_64.contains_key(k) {
                let v = &self.int_map_64[k];
                println!("{}", v);
                return Ok(());
            }

            let err_text = format!("{}:{} {} is not defined.\n", token.loc.row, token.loc.col, k);
            return Err(make_error(&err_text));
        }

        return Ok(());
    }
}

