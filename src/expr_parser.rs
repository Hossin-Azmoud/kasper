use std::io;

use crate::lexer::{ KasperLexer, match_lexer_token };
use crate::token::Token;
use crate::util::make_error;
use crate::enums::{ TokenT, make_prec_table };
use std::collections::HashMap;
use crate::stack::*;


pub struct ArithmaticParser {
    pub iostack: Vec<Token>,
    pub prec_table: HashMap<String, i32>
}

fn is_parent(operand: TokenT) -> bool {
    return operand == TokenT::OPAR__ || operand == TokenT::CPAR__;
}

impl ArithmaticParser {
    
    pub fn new() -> Self {
        ArithmaticParser { 
            iostack: Vec::new(),
            prec_table: make_prec_table(),
        }
    }

    pub fn _dump_stack(&mut self) {
        for i in 0..self.iostack.len() {
            print!("{} ", self.iostack[i].value);
            
        }
        println!();
    }

    pub fn _dump_res_stack(&mut self, res: &Vec<f64>) {
        for i in 0..res.len() {
            print!("{} ", res[i]);
            
        }
        println!();
    }

    pub fn read_expression(&mut self, lex: &mut KasperLexer, mem_stack: &mut Stack) -> Result<f64, io::Error> {
        match self.postfix(lex) {
            Ok(()) => {
                match self.evaluate(mem_stack) {
                    Ok(v)  => Ok(v),
                    Err(e) => return Err(e),

                }
            },
            Err(e) => return Err(e),
        }
    }

    pub fn postfix(&mut self, lex: &mut KasperLexer) -> Result<(), io::Error> {
        let mut tmp: Vec<Token> = Vec::new();        
      
        while lex.is_not_empty() {     
            let token = match_lexer_token(lex.next()); // Next Token.
            let prev = tmp.last_mut().cloned(); // An Option...
 
            if token.token_type == TokenT::NL__ {
                break;
            }
            
            if token.token_type == TokenT::COMMENT__ {
                continue;
            }

            if token.token_type == TokenT::NUMBER__ || token.token_type == TokenT::VARNAME__ {
                self.iostack.push(token);
                continue;
            }
            
            if token.token_type == TokenT::OPAR__ {
                tmp.push(token);
                continue;
            }

            if token.token_type == TokenT::CPAR__ {
                if let Some(mut v) = prev {
                    while v.token_type != TokenT::OPAR__ && tmp.len() > 0 {

                        self.iostack.push(v.clone());
                        tmp.pop();
                        
                        if let Some(other) = tmp.last_mut().cloned() {
                            v = other; // Move..
                            continue;
                        }

                        break;

                    }
                    
                    if let Some(other) = tmp.last_mut().cloned() {
                        if other.token_type == TokenT::OPAR__ {
                            tmp.pop();
                            continue;
                        }
                    }
                }

                let err = format!("{}:{}:{} Non-closed bracket Error.", lex.file_path, token.loc.row, token.loc.col);
                return Err(make_error(&err));
            }
            

            if self.is_in_prec(&token.value) {
                if let Some(mut v) = prev {
                    let c: &String = &v.value;
                    
                    if self.is_in_prec(c) {
                        if self.is_greater(&token.value, c) {
                            tmp.push(token);
                            continue;
                        }
                        
                        while self.is_less_eq(&token.value, &v.value) && v.value != token.value {
                            
                            self.iostack.push(v.clone());
                            tmp.pop();
                            if let Some(n) = tmp.last_mut().cloned() {
                                v = n;
                                if !self.is_in_prec(&v.value) {
                                    break;
                                }

                                continue;
                            }

                            break;
                        }

                    }
                }
                
                tmp.push(token);
            }
            

        }
        
        while tmp.len() > 0 {
            if let Some(v) = tmp.pop() {
                self.iostack.push(v);
                continue;
            }
            
            break;
        }
        
        for i in 0..self.iostack.len() {
            if is_parent(self.iostack[i].token_type) {
                let err = format!("{}:{}:{} Non-closed bracket Error.", lex.file_path, lex.row, lex.col);
                return Err(make_error(&err));
            }
        }
        
        self.iostack.reverse();
        Ok(())
    }
    
    pub fn is_greater(&mut self, f: &String, s: &String) -> bool {
        return self.prec_table[f] > self.prec_table[s];
    }
    pub fn is_less_eq(&mut self, f: &String, s: &String) -> bool {
        return self.prec_table[f] <= self.prec_table[s];
    }

    pub fn is_in_prec(&mut self, s: &String) -> bool {
        return self.prec_table.contains_key(s);
    }
/*
    pub fn clear_stack(&mut self) {
        self.iostack = Vec::new();
    }
*/    
    pub fn evaluate(&mut self, mem_stack: &mut Stack) -> Result<f64, io::Error> {
        let mut res: Vec<f64> = Vec::new();
 

        while self.iostack.len() > 0 {
            
            let l: Token = self.iostack.pop().unwrap();
            // oprnd
            if self.is_in_prec(&l.value) {
                
                let a: f64 = res.pop().unwrap();
                let b: f64 = res.pop().unwrap();
            
                if l.token_type == TokenT::PLUS__ {
                    res.push(a + b);  
                    continue;    
                } 
                
                if l.token_type == TokenT::MINUS__ {
                    res.push(b - a);  
                    continue;    
                } 
                
                if l.token_type == TokenT::MULT__ {
                    res.push(a * b);  
                    continue;    
                } 
                
                if l.token_type == TokenT::DIV__ {
                    
                    if a == 0.0 {
                        let err = format!("{}:{} Division by zero Error.", 
                                    l.loc.row, 
                                    l.loc.col
                            );

                        return Err(make_error(&err));
                    }
                    
                    res.push(b / a);  
                    continue;    
                } 

                if l.token_type == TokenT::POW__ {
                    res.push(b.powf(a));
                    continue;
                } 
                
                let err = format!("{}:{} Unsupported operand {}", 
                              l.loc.row, 
                              l.loc.col, 
                              l.value
                        );

                return Err(make_error(&err));
            }
            

            if l.token_type == TokenT::NUMBER__ || l.token_type == TokenT::FLOAT__ {
                res.push(l.value.parse::<f64>().unwrap());
                continue;
            }
            
            let undef = format!("{}:{} {} is undefined !",
                            l.loc.row,
                            l.loc.col,
                            l.value
                        );

            if l.token_type == TokenT::VARNAME__ {
                // find the value then push it..
                match mem_stack.get_int(&l.value) {
                    
                    Some(val) => {
                        res.push(val);
                        continue;
                    },

                    None      => return Err(make_error(&undef))
                }
            }
        }

        if let Some(value) = res.last() {
            return Ok(*value);
        }
        
        let err = format!("Value was lost in the stack.");
        return Err(make_error(&err));
    }
}










