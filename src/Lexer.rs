#[allow(non_snake_case, dead_code)]
// STD
use std::fs::File;
use std::io;
use std::io::Read;
use std::collections::HashMap;
use std::process::exit;

// CUSTOM.
use crate::enums::*;
use crate::token::*;
use crate::util::make_error;

pub struct KasperLexer<'a> {
    pub file_path: &'a str,
    pub source:    Vec<u8>,
    pub cur:       usize,
    pub row:       usize,
    pub col:       usize,
    pub size:      usize,
    token_table: HashMap<char, TokenT>,
} 


#[allow(dead_code)]
impl<'a> KasperLexer<'a> {
    
    pub fn new(path: &'a str) -> Self    
    {
        return KasperLexer {
            file_path: path,
            source:    vec![],
            cur:       0,
            row:       1,
            col:       1,
            size:      0,
            token_table: make_token_table(),
        };
    }


    pub fn get_char(&mut self, index: usize) -> char { 
        
        if self.is_not_empty()
        {
            return char::from(self.source[index]);
        }

        return '\0';
    }
    
    pub fn get_prev(&mut self) -> char {
        if self.cur > 0 {
            return self.get_char(self.cur - 1);
        } else {
            return self.get_char(0);
        }
    }

    pub fn get_current(&mut self) -> char {
        return self.get_char(self.cur);
    }
    
    pub fn get_next(&mut self) -> char {
        return self.get_char(self.cur + 1);
    }

    pub fn is_not_empty(&mut self) -> bool {
        return self.cur < self.size;
    }
    
    pub fn display(&mut self) {     
        println!("fp: {}", self.file_path);
        println!("sourcelen: {}", self.source.len());
        println!("cur: {}", self.cur);
        println!("row: {}", self.row);
        println!("col: {}", self.col);
    }

    pub fn chop(&mut self) -> usize {
        if self.is_not_empty()  {
            self.cur += 1;
    
            let c: char = self.get_current();
                        
            if c == NL {
                self.row  += 1;
                self.col =  0;
                 
                return self.cur;
            }
            
            self.col += 1;
        }

        return self.cur;
    }
        
    pub fn read(&mut self) -> io::Result<()> {
        let mut tmp = File::open(self.file_path)?; 
        tmp.read_to_end(&mut self.source)?;
        self.size = self.source.len();        
        Ok(())
    }
/*    
    pub fn read_block(&mut self) -> io::Result<Token, io::Error> {
        let mut token: Token = match_lexer_token(self.next());
        if token.token_type != TokenT::OCURLY__ {
            let err = format!("READING THE BLOCK FAILED. read_block()\r reason: You need to read in a state of block start {");
            return Err(make_error(&err));
        } 
        
        let mut opening_loc = token.loc.clone();
        
        if token.token_type != TokenT::CCURLY__ && self.is_not_empty() {
            token = match_lexer_token(self.next());
        }
                
    }
    
    pub fn skip_block(&mut self) -> io::Result<(), io::Error> {
        let mut token: Token = match_lexer_token(self.next());
        let mut opening_loc = token.loc.clone();

        if token.token_type != TokenT::OCURLY__ {
            let err = format!("READING THE BLOCK FAILED. read_block()\r reason: You need to read in a state of block start {");
            return Err(make_error(&err));
        } 
        
        while token.token_type != TokenT::CCURLY__ && self.is_not_empty() {
            token = match_lexer_token(self.next()); 
        }

        if token.token_type != TokenT::CCURLY__ {
            
            let err = format!("{}:{}:{} unclosed scope", 
                              self.file_path, 
                              opening_loc.row, 
                              opening_loc.col
                       );

            return Err(make_error(&err));
        }
        
    }
*/   
    pub fn handle_comment(&mut self,token: &mut Token)  -> Result<(), io::Error> {
        let mut c: char = self.get_current();
        
        // check for '/' in the next char, if it is not '/' then error.
        
        if c == DIV {
            token.token_type = TokenT::COMMENT__; 

            while c != NL {
                token.write(c);
                self.chop();
                c = self.get_current();
            }
        }

 
        // COMMENT
        return Ok(());
    }
    
    pub fn write_to_token(&mut self, token: &mut Token, type_: TokenT, c: char) { 
        token.write(c);
        token.token_type = type_;
        self.chop();
    }

    pub fn write_to_special_token(&mut self, token: &mut Token, c: char) {
        if self.token_table.contains_key(&c) {
            let t = self.token_table[&c];
            
            if t == TokenT::MINUS__ {
                if self.get_next() == GT {
                    // ->
                    token.write(c);
                    self.chop();

                    self.write_to_token(token, TokenT::THIN_ARROW__, GT);
                    return;
                }
            }
            
            if t == TokenT::EQUAL__ {
                if self.get_next() == GT {
                    // =>
                    token.write(c);
                    self.chop();

                    self.write_to_token(token, TokenT::FAT_ARROW__, GT);
                    
                    return;
                }
            }

                        
            if t != TokenT::DQUOTE__ {
                self.write_to_token(token, t, c);
                return;
            }

            token.token_type = t;
            self.chop();
            return;
        }

        println!("USED FUNC(write_to_special_token) WITH UNKNOWN SPECIAL TOKN.");
    }

    pub fn match_current(&mut self, token: &mut Token) -> Result<(), io::Error> {
        let mut c: char = self.get_current();
        // it is a known token.
        token.loc.change_loc(self.row, self.col);
        
        if self.token_table.contains_key(&c) {
            self.write_to_special_token(token, c);
            
            if token.token_type == TokenT::DIV__ {
                let result = self.handle_comment(token);
            }
            
            return Ok(());
        }

        if c.is_ascii_punctuation() {
                       
            token.write(c);
            token.token_type = TokenT::NONE__;
            self.chop();
        
        }

        return Ok(());
    }
 
    pub fn trim_spaces_left(&mut self)  {
        
        if self.is_not_empty() {
            
            let mut c = char::from(self.source[self.cur]);

            while c.is_ascii_whitespace() && self.is_not_empty() {
                if c == NL {
                    break;
                }
                
                self.chop();
                
                if self.cur < self.size {
                    c = char::from(self.source[self.cur]);
                    continue;
                }

                return;
            }
        }

    }

    pub fn collect_str(&mut self, token: &mut Token){
        token.token_type = TokenT::VARNAME__;
        let mut c: char =  self.get_current();

        while c.is_alphanumeric() || c.is_digit(10) && self.is_not_empty() {
            if c.is_ascii_punctuation() {
                break;
            }
            
            if c.is_ascii_whitespace() {
                break;
            }

            token.write(c);
            self.chop();
            c = self.get_current();
        }
    }
    
    pub fn collect_number(&mut self, token: &mut Token) { 
        token.token_type = TokenT::NUMBER__;
        let mut c: char = self.get_current();
        while self.is_not_empty() && c.is_digit(10) {
            
            if c.is_ascii_punctuation() {
                break;
            }

            if c.is_ascii_whitespace() {
                break;
            }
            
            token.write(c);
            self.chop();
            c = self.get_current();
        }
    }

    pub fn next(&mut self) -> Result<Token, io::Error> {
        
        self.trim_spaces_left();
    
        let mut token = Token::empty();
        
        // TODO: Match with already defined tokens.
        let res = self.match_current(&mut token);

        match res {
            Err(e) => return Err(e),
            Ok(()) => {
                let mut prev: char = self.get_prev();
                let mut c: char    = self.get_current();
                
                if token.token_type == TokenT::DQUOTE__ {
                    while self.is_not_empty() {
                        if prev == ESCAPE {
                            match c {
                                '\"' => {
                                    token.write(DQUOTE);
                                },
                                '\'' => {
                                    token.write(SQUOTE);

                                },
                                '\\' => {
                                    token.write(ESCAPE);
                                },
                                'n' => {
                                    token.write(NL);
                                },
                                't' => {
                                    token.write(TAB);
                                },
                                'r' => {
                                    token.write(RE);
                                },
                                '0' => {
                                    token.write(NULLC);
                                },
                                _ => {
                                    todo!("Unreachable!, if the escape sequence is not completed.");
                                }
                            }
                            
                            self.chop();
                            prev = c.clone();
                            c = self.get_current();
                        }

                        if c == DQUOTE {
                            token.token_type = TokenT::STRING__;
                            self.chop();
                            return Ok(token);
                        }
                        if c != ESCAPE {
                            token.write(c);
                        }
                        
                        self.chop();
                        prev = c.clone();
                        c = self.get_current();
                    }
                    
                    // We did not find the terminating quote ?
                    let mut err_text = format!("{}:{}:{} Interminated string literal.", self.file_path, token.loc.row, token.loc.col);
                    err_text    += &format!("Add \" to terminate the string..");
                    return Err(make_error(&err_text));
                }
            
                if token.size > 0 { 
                    return Ok(token);
                }
                 
                token.loc.change_loc(self.row, self.col);
                
                if c.is_alphabetic() {
                    self.collect_str(&mut token); // VARNAME__
                    
                    match &token.value as &str {
                        DEFINE => {
                            token.token_type = TokenT::KEY_WORD_DEFINE__;
                            return Ok(token);
                        },
                        INT => {
                            token.token_type = TokenT::INT_T;
                            return Ok(token);
                        },
                        STRING => {
                            token.token_type = TokenT::STRING_T;
                            return Ok(token);
                        },
                        INT_64 => {
                            token.token_type = TokenT::INT_T_64;
                            return Ok(token);
                        },
                        BOOL => {
                            token.token_type = TokenT::BOOL_T;
                            return Ok(token);
                        },
                        BOOL_TRUE => {
                            token.token_type = TokenT::BOOL_TRUE__;
                            return Ok(token);

                        },
                        BOOL_FALSE => {
                            token.token_type = TokenT::BOOL_FALSE__;
                            return Ok(token);
                        },
                        IF => {
                            token.token_type = TokenT::IF__;
                            return Ok(token);
 
                        },
                        ELSE => {
                            token.token_type = TokenT::ELSE__;
                            return Ok(token);
 
                        },
                        COMP_EQ => {
                            token.token_type = TokenT::COMP_EQ__;
                            return Ok(token);
 
                        },
                        COMP_NOT_EQ => {
                            token.token_type = TokenT::COMP_NOT_EQ__;
                            return Ok(token);
 
                        },
                        COMP_LT_EQ => {
                            token.token_type = TokenT::COMP_LT_EQ__;
                            return Ok(token);
 
                        },
                        COMP_GT_EQ => {
                            token.token_type = TokenT::COMP_GT_EQ__;
                            return Ok(token);
 
                        }
                        _ => {
                            if self.get_current() == OPAR {
                                token.token_type = TokenT::FUNC_CALL__;
                                self.chop();
                                return Ok(token);
                            }
                        }
                    }
                }
                
                if c.is_digit(10) {
                    self.collect_number(&mut token);
                }

                return Ok(token);
            }
        }
    }
}


pub fn match_lexer_token(res: Result<Token, io::Error>) -> Token {
    match res {
        Ok(token) => {
            return token;        
        },
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

}

