// TODO: Parse complex arithmatics.
// TODO: 
use std::io;
use crate::enums::*;
use crate::stack::*;
use crate::lexer::{ KasperLexer, match_lexer_token };
use crate::util::{ make_error }; // not_implemented
use crate::token::Token;
use crate::expr_parser::ArithmaticParser;

#[allow(dead_code)]
pub struct Variable {
    name:             String,
    value:            String,
    var_parsed_type:  TokenT,
    declared_type:    TokenT,
}

#[allow(dead_code)]
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

    pub fn is_number(&mut self) -> bool {
        return self.declared_type == TokenT::FLOAT__  || self.declared_type == TokenT::INT_T  || self.declared_type == TokenT::INT_T_64;
    }
}

// STACK MAPS:

/*
 
str_map:           HashMap<String, String>,
int_map_8:         HashMap<String, i8>, // 8 bit..
int_map_16:        HashMap<String, i16>, // 16 bit..
int_map:           HashMap<String, i32>, // 32 bit..
int_map_64:        HashMap<String, i64>, // 64 bit..
int_map_128:       HashMap<String, i128>, // 128 bit..
int_map_8_array:   HashMap<String, Vec<i8>>, // 8 bit..
int_map_16_array:  HashMap<String, Vec<i16>>, // 16 bit..
int_map_array:     HashMap<String, Vec<i32>>, // 32 bit..
int_map_64_array:  HashMap<String, Vec<i64>>, // 64 bit..
int_map_128_array: HashMap<String, Vec<i128>>, // 128 bit..  


*/

pub struct KasperParser<'a> {
    pub lexer: KasperLexer<'a>, // Reads the code.. 
    pub ap: ArithmaticParser,   // Read math expressions...
    stack: Stack,               // Stores variables...
}

#[allow(unreachable_patterns, dead_code)]  
impl<'a> KasperParser<'a> {
    
     pub fn new(lex: KasperLexer<'a>) -> Self {
        Self {
            lexer: lex,
            stack: Stack::new(),
            ap: ArithmaticParser::new(),
        }
    }
    
        
    pub fn parse_lexer(&mut self) -> Result<(), io::Error> {
        let mut token = match_lexer_token(self.lexer.next());        

        if token.token_type == TokenT::NONE__ {
            
            let err = format!("{}:{}:{} Syntax error, unrecognized token { }",
                               self.lexer.file_path, 
                               token.loc.col, 
                               token.loc.row, 
                               token.value  
                            );

            return Err(make_error(&err));
        }
        
        if token.token_type == TokenT::NL__ {
           return Ok(());
        }

        if token.token_type != TokenT::COMMENT__ { 
            
            if token.token_type == TokenT::FUNC_CALL__ {
                
                if token.value == String::from(WRITE) {
                    return self.parse_print();
                }

                let err = format!("{}:{}:{} {} is not defined", 
                                  self.lexer.file_path, 
                                  token.loc.col, 
                                  token.loc.row, 
                                  token.value
                            );

                return Err(make_error(&err));
            }

            if token.token_type == TokenT::KEY_WORD_DEFINE__ {
                return self.parse_def();
            }

            if token.token_type == TokenT::VARNAME__ { // var
                return self.assign_variable(&mut token); 
            }
          
            return Ok(());
        }

        return Ok(());
    
    }
    
    // pub fn parse_condition(&mut self) -> Result<bool, io::Error> {
    //     let mut parsed_condition: bool = true;
    //     let token: Token           = match_lexer_token(self.lexer.next());
        
    //     if token.token_type == TokenT::VARNAME__ {
    //         // Parse the codition that is inside.
    //         if self.stack.defined(&token.value) {
    //             // | x |

    //             if self.lexer.get_next() == PIPE { 
    //                 self.lexer.chop(); // Chop the pipe.
    //                 if let Some(v) = self.stack.get_from_bool_map(&token.value) {
    //                     return Ok(*v);
    //                 }

    //             }
                
    //             let err = format!("{}:{}:{} expected a PIPE (|) but found {}", 
    //                       self.lexer.file_path, 
    //                       token.loc.row, 
    //                       token.loc.col,
    //                       token.value
    //                 );

    //             return Err(make_error(&err));
    //         }

    //         let err = format!("{}:{}:{} {} is Undefined", 
    //                   self.lexer.file_path, 
    //                   token.loc.row, 
    //                   token.loc.col, 
    //                   token.value
    //             );

    //         return Err(make_error(&err));
    //     }
        
    //     if token.token_type == TokenT::NUMBER__  {
            
    //         not_implemented("Condition are not Implemented yet!");
    //         let lhs: i128 = token.value.parse::<i128>().unwrap(); // LHS
    //         let tmp = match_lexer_token(self.lexer.next());
            
    //     }
        
    //     if token.token_type == TokenT::STRING__  {
    //         not_implemented("Condition are not Implemented yet!");
    //     }
        
    //     return Ok(true);
    // }

    // pub fn parse_branching(&mut self) -> Result<bool, io::Error>{
    //     // if | x == 0 | { ... } else { ... }
    //     let token = match_lexer_token(self.lexer.next());    
        
    //     if token.token_type == TokenT::PIPE__ {
    //         match self.parse_condition() {
    //             Ok(v) => return Ok(v),
    //             Err(e) => return Err(e),
    //         }
    //     }
        
    //     let err = format!("{}:{}:{} expected a pipe | but got {} instead", self.lexer.file_path, token.loc.row, token.loc.col, token.value);
    //     return Err(make_error(&err));
    // }
    
    pub fn assign_variable(&mut self, token: &mut Token) -> Result<(), io::Error> {
        
        let variable_name: &String = &token.value.clone();
        
        if self.stack.defined(&token.value) {    
            if match_lexer_token(self.lexer.next()).token_type == TokenT::EQUAL__ {
                // Assign variable. = val.
                self.lexer.chop();
                *token = match_lexer_token(self.lexer.next()); // Value..
                if token.token_type == TokenT::NUMBER__ {
            
                    if self.stack.int_map.contains_key(variable_name) { 
                        let v = token.value.parse::<i32>().unwrap();            
                        self.stack.push_int_map(variable_name, v);
                        return Ok(());
                    }     
                
                    if self.stack.int_map_64.contains_key(variable_name) {
                        let v = token.value.parse::<i64>().unwrap();            
                        self.stack.push_int_map_64(variable_name, v);
                        return Ok(());
                    }
                     
                }        
        
                if token.token_type == TokenT::STRING__ {
                    if self.stack.str_map.contains_key(variable_name) {
                        self.stack.push_str_map(variable_name, token.value.clone());
                        return Ok(());
                    } 
                }
        
                if token.token_type == TokenT::BOOL_FALSE__ || token.token_type == TokenT::BOOL_TRUE__ {
                    let v: bool = token.token_type == TokenT::BOOL_TRUE__;
            
                    if self.stack.bool_map.contains_key(variable_name) {
                        self.stack.push_bool_map(variable_name, v); // True or False
                        return Ok(());
                    } 

                }

                let err = format!("{}:{}:{} unexpected token {}", self.lexer.file_path, token.loc.row, token.loc.col, token.value);
                return Err(make_error(&err));
            }
                
            let err = format!("{}:{}:{} expected = got {} instead.", self.lexer.file_path, token.loc.col, token.loc.row, token.value);
            return Err(make_error(&err));


        }

        let err = format!("{}:{}:{} {} is not defined try define {} -> T", self.lexer.file_path, token.loc.col, token.loc.row, token.value,token.value);
        return Err(make_error(&err));         
    }

    pub fn parse_lhs(&mut self) -> Result<Variable, io::Error> {
        // Parse var -> T
        let mut var   = Variable::empty();
        let mut token = match_lexer_token(self.lexer.next());  // Get VarName.
        
        if token.token_type == TokenT::VARNAME__ {
            var.name = token.value;
            token = match_lexer_token(self.lexer.next()); // ->
            if token.token_type == TokenT::THIN_ARROW__ {
                token = match_lexer_token(self.lexer.next()); // T 
                
                if token.token_type == TokenT::STRING_T {
                    var.declared_type = TokenT::STRING_T;
                    
                    self.stack.push_str_map(&var.name, "".to_string()); // Add var with name
                                                                              // and default value
                    return Ok(var);
                }

                if token.token_type == TokenT::INT_T {
                    var.declared_type = TokenT::INT_T;
                    self.stack.push_int_map(&var.name, 0);
                    return Ok(var);
                }

                if token.token_type == TokenT::INT_T_64 {
                    var.declared_type = TokenT::INT_T_64;
                    self.stack.push_int_map_64(&var.name, 0);
                    return Ok(var);

                }
                
                if token.token_type == TokenT::BOOL_T {
                    var.declared_type = TokenT::BOOL_T;
                    self.stack.push_bool_map(&var.name, true);
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
    

    pub fn register_var(&mut self, mut variable: Variable) -> Result<(), io::Error> {

        if variable.declared_type == TokenT::STRING_T  {
            let token = match_lexer_token(self.lexer.next());
            
            if token.token_type == TokenT::STRING__ {
                self.stack.push_str_map(&variable.name, token.value);
                return Ok(());
            }

            let err = format!("{}:{}:{} unexpected token, expected string but found {}",
                        self.lexer.file_path,
                        token.loc.row,
                        token.loc.col,
                        token.token_type
                    );

            return Err(make_error(&err));
        } 
        
        if variable.declared_type == TokenT::BOOL_T {
            let token = match_lexer_token(self.lexer.next());

            if token.token_type == TokenT::BOOL_TRUE__ || token.token_type == TokenT::BOOL_FALSE__ {
                let v = token.token_type == TokenT::BOOL_TRUE__;
                self.stack.push_bool_map(&variable.name, v);
                return Ok(());
            }

            let err = format!("{}:{}:{} unexpected token, expected boolean but found {}",
                        self.lexer.file_path,
                        token.loc.row,
                        token.loc.col,
                        token.token_type
                );

            return Err(make_error(&err));
        }


        if variable.is_number() {
            match self.ap.read_expression(&mut self.lexer, &mut self.stack)
            {
                Ok(v) => {

                    if variable.declared_type == TokenT::INT_T { // Reg as i32
                        self.stack.push_int_map(&variable.name, v as i32);
                        return Ok(());
                    }
                    
                    if variable.declared_type == TokenT::INT_T_64 { // Reg as i64
                        self.stack.push_int_map_64(&variable.name, v as i64);
                        return Ok(());
                    }
                    
                    let err = format!("unsupported type {}", variable.declared_type);
                    return Err(make_error(&err));
                },

                Err(e) => return Err(e),
            };
        };
        
                
        let err = format!("{}:{}:{} Syntax err", 
                    self.lexer.file_path,
                    self.lexer.row,
                    self.lexer.col
                );

        return Err(make_error(&err));
    }

    pub fn parse_def(&mut self) -> Result<(), io::Error> {
        
        match self.parse_lhs() {
            Ok(variable) => {
                // successs parsing the type..
                if match_lexer_token(self.lexer.next()).token_type == TokenT::EQUAL__ {
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
                        
            if self.stack.print_variable(k) {
                return Ok(());
            }

            let err_text = format!("{}:{} {} is not defined.\n", 
                                   token.loc.row, 
                                   token.loc.col, 
                                   k);
            return Err(make_error(&err_text));
        }

        return Ok(());
    }
}
