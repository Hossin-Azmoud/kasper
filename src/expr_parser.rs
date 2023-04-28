use std::io;

use crate::lexer::{ KasperLexer, match_lexer_token };
use crate::token::Token;
use crate::util::make_error;
use crate::enums::{ TokenT, make_prec_table };
use std::collections::HashMap;

pub struct ArithmaticParser<'a> {
    pub lex: &'a mut KasperLexer<'a>,
    pub IOStack: Vec<Token>,
    pub PrecTable: HashMap<String, i32>
}
fn is_parent(T: TokenT) -> bool {
    return T == TokenT::OPAR__ || T == TokenT::CPAR__;
}

impl<'a> ArithmaticParser<'a> {
    
    pub fn new(lexer: &'a mut KasperLexer<'a>) -> Self {
        ArithmaticParser {
            lex: lexer,
            IOStack: Vec::new(),
            PrecTable: make_prec_table(),
        }
    }

    pub fn dump_stack(&mut self) {
        for i in 0..self.IOStack.len() {
            print!("{} ", self.IOStack[i].value);
            
        }
        println!();
    }

    pub fn postfix(&mut self) -> Result<(), io::Error> {
        let mut tmp: Vec<Token> = Vec::new();
        
        while self.lex.is_not_empty() {
            let mut token = match_lexer_token(self.lex.next());
            let mut prev = tmp.last_mut().cloned(); // An Option...
            self.dump_stack();
            if token.token_type == TokenT::COMMENT__ {
                
                continue;
            }

            if token.token_type == TokenT::NUMBER__ {
                self.IOStack.push(token);
                continue;
            }
            
            if token.token_type == TokenT::OPAR__ {
                tmp.push(token);
                continue;
            }

            if token.token_type == TokenT::CPAR__ {
                if let Some(mut v) = prev {
                    while v.token_type != TokenT::OPAR__ && tmp.len() > 0 {

                        self.IOStack.push(v.clone());
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

                let err = format!("{}:{}:{} Non-closed bracket Error.", self.lex.file_path, token.loc.row, token.loc.col);
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
                            
                            self.IOStack.push(v.clone());
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
                self.IOStack.push(v);
                continue;
            }
            
            break;
        }
        
        for i in 0..self.IOStack.len() {
            if is_parent(self.IOStack[i].token_type) {
                let err = format!("{}:{}:{} Non-closed bracket Error.", self.lex.file_path, self.lex.row, self.lex.col);
                return Err(make_error(&err));
            }
        }
        
        self.IOStack.reverse();
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
        self.IOStack = Vec::new();
    }
    
    pub fn evaluate(&mut self) -> Result<(), io::Error> {
        Ok(())
    }

}










