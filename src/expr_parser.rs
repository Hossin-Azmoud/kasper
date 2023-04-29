use std::io;

use crate::lexer::{ KasperLexer, match_lexer_token };
use crate::token::Token;
use crate::util::make_error;
use crate::enums::{ TokenT, make_prec_table };
use std::collections::HashMap;

pub struct ArithmaticParser {
    pub iostack: Vec<Token>,
    pub PrecTable: HashMap<String, i32>
}
fn is_parent(T: TokenT) -> bool {
    return T == TokenT::OPAR__ || T == TokenT::CPAR__;
}

impl ArithmaticParser {
    
    pub fn new() -> Self {
        ArithmaticParser { 
            iostack: Vec::new(),
            PrecTable: make_prec_table(),
        }
    }

    pub fn dump_stack(&mut self) {
        for i in 0..self.iostack.len() {
            print!("{} ", self.iostack[i].value);
            
        }
        println!();
    }

    pub fn postfix(&mut self, lex: &mut KasperLexer) -> Result<(), io::Error> {
        let mut tmp: Vec<Token> = Vec::new();

        while lex.is_not_empty() {
             
            let mut token = match_lexer_token(lex.next());
            if token.token_type == TokenT::NL__ {
                break;
            }
            
            let mut prev = tmp.last_mut().cloned(); // An Option...
            // self.dump_stack();
            if token.token_type == TokenT::COMMENT__ {
                continue;
            }

            if token.token_type == TokenT::NUMBER__ {
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

        return self.PrecTable[f] > self.PrecTable[s];
    }
    pub fn is_less_eq(&mut self, f: &String, s: &String) -> bool {
        return self.PrecTable[f] <= self.PrecTable[s];
    }

    pub fn is_in_prec(&mut self, s: &String) -> bool {
        return self.PrecTable.contains_key(s);
    }

    pub fn clear_stack(&mut self) {
        self.iostack = Vec::new();
    }
    
    pub fn evaluate(&mut self) -> Result<f64, io::Error> {
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
                        return Err(make_error("Division by zero Error."));
                    }
                    
                    res.push(b / a);  
                    continue;    
                } 

                if l.token_type == TokenT::POW__ {
                    res.push(b.powf(a));
                    continue;
                } 
                
                let err = format!("Unsupported operand {}", l.value);
                return Err(make_error(&err));
            }
            // Number.
            res.push(l.value.parse::<f64>().unwrap());
        }

        if let Some(value) = res.last() {
            return Ok(*value);
        }

        let err = format!("Value was lost in the stack.");
        return Err(make_error(&err));
    }

}










