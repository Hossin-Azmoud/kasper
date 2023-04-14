
#[allow(non_snake_case, dead_code)]
use std::fs::File;
use std::io;
use std::io::Read;
use std::fmt;
use std::collections::HashMap;
use std::process::exit;

pub const DQUOTE:   char   = '\"';
pub const SQUOTE:   char   = '\'';
pub const SPACE:     char  = ' ';
pub const NL:        char  = '\n';
pub const OPAR:      char  = '(';
pub const CPAR:      char  = ')';
pub const OCURLY:    char  = '{';
pub const CCURLY:    char  = '}';
pub const PLUS:      char  = '+';
pub const MINUS:     char  = '-';
pub const COMA:      char  = ',';
pub const SEMICOLON: char  = ';';
pub const EQUAL:     char  = '=';
pub const GT:        char  = '>';
pub const LT:        char  = '<';
pub const QM:        char  = '!';
pub const COMMENT:   char  = '/';
pub const ESCAPE:    char  = '\\';
pub const PRINT:     &str = "print";

pub fn make_error(text: &str) -> io::Error { 
    return io::Error::new(io::ErrorKind::Other, text);    
}

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]

pub enum TokenT {
    // Special tokens.
    COMMENT__,
    DQUOTE__,
    SQUOTE__,
    OPAR__,
    CPAR__,
    OCURLY__,
    CCURLY__,
    PLUS__,
    MINUS__,
    COMA__,
    SEMICOLON__,
    EQUAL__,
    GT__,
    LT__,
    QM__,

    // Other
    NONE__,
    NUMBER__,
    STRING__,
    VARNAME__,
    
    // Built-in functions.
    PRINT__,
}

fn make_token_table() -> HashMap<char, TokenT> {
    let mut map: HashMap<char ,TokenT> = HashMap::new();
    
    //Adding all the keys. 
    map.insert(DQUOTE, TokenT::DQUOTE__);
    map.insert(SQUOTE, TokenT::SQUOTE__);
    map.insert(OPAR, TokenT::OPAR__);
    map.insert(CPAR, TokenT::CPAR__);
    map.insert(OCURLY, TokenT::OCURLY__);
    map.insert(CCURLY, TokenT::CCURLY__);
    map.insert(PLUS, TokenT::PLUS__);
    map.insert(MINUS, TokenT::MINUS__);
    map.insert(COMA, TokenT::COMA__);
    map.insert(SEMICOLON, TokenT::SEMICOLON__);
    map.insert(EQUAL, TokenT::EQUAL__);
    map.insert(GT, TokenT::GT__);
    map.insert(LT, TokenT::LT__);
    map.insert(QM, TokenT::QM__);

    // Return the map.
    map
}

impl fmt::Display for TokenT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let printable = match *self {
            TokenT::NONE__      => "NONE__",
            TokenT::DQUOTE__    => "DQUOTE__",
            TokenT::SQUOTE__    => "SQUOTE__",
            TokenT::OPAR__      => "OPAR__",
            TokenT::CPAR__      => "CPAR__",
            TokenT::OCURLY__    => "OCURLY__",
            TokenT::CCURLY__    => "CCURLY__",
            TokenT::PLUS__      => "PLUS__",
            TokenT::MINUS__     => "MINUS__",
            TokenT::COMA__      => "COMA__",
            TokenT::SEMICOLON__ => "SEMICOLON__",
            TokenT::EQUAL__     => "EQUAL__",
            TokenT::GT__        => "GT__",
            TokenT::LT__        => "LT__",
            TokenT::NUMBER__    => "NUMBER__",
            TokenT::STRING__    => "STRING__",
            TokenT::QM__        => "QM__",
            TokenT::VARNAME__   => "VARNAME__",
            TokenT::COMMENT__   => "COMMENT__",
            TokenT::PRINT__     => "PRINT__",
        }; 
     
        write!(f, "{}", printable)
    }
}

pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
    
    pub fn empty() -> Self {
        Location {
            row: 0,
            col: 0,
        }
    }

    pub fn change_loc(&mut self, row: usize, col: usize) {
        // The indexing of raws and cols start from 1, so we need to increment it.
        self.row = row;
        self.col = col;
    }

}

pub struct Token {
    pub  value:       String,
    pub  token_type:  TokenT,
    pub  size:        usize,
    pub  loc:         Location,
}

impl Token {

    pub fn empty() -> Self {
        Token {
            value: String::from(""),
            token_type: TokenT::NONE__,
            size: 0,
            loc: Location::empty(),
        }
    }
    
    
    pub fn write(&mut self, c: char) {
        self.value += &String::from(c);
        self.size += 1;
    }
    
    pub fn display_token(&mut self) {
        println!("r: {}", self.loc.row);
        println!("c: {}", self.loc.col);
        println!("t: {}", self.token_type);
        println!("v: {}", self.value);

        println!();  
    }
}

pub struct KasperLexer<'a> {
    pub file_path: &'a str,
    pub source:    Vec<u8>,
    pub cur:       usize,
    pub row:       usize,
    pub col:       usize,
    pub size:      usize,
    token_table: HashMap<char, TokenT>,
} 


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
        
        if self.is_not_empty() {
            return char::from(self.source[index]);
        }

        return '\0';
    }
    
    pub fn get_prev(&mut self) -> char {
        return self.get_char(self.cur - 1);
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
    
    pub fn make_error(&mut self, text: &str) -> io::Error { 
        return io::Error::new(io::ErrorKind::Other, text);
    
    }
    
    pub fn handle_comment(&mut self, c: &mut char,token: &mut Token)  -> Result<(), io::Error> {
        
        // write '/' then remove it?
        token.token_type = TokenT::COMMENT__;
        token.write(*c);
        self.chop();

        *c = self.get_current();
        
        // check for '/' in the next char, if it is not '/' then error.
        
        if *c != COMMENT {
            let mut err_text = format!("expected // but found | {} |\n", *c);
            err_text     += &format!("you can solve this by replacing {} with //", *c);
            return Err(make_error(&err_text));
        }
 
        // COMMENT
        while *c != NL {
            token.write(*c);
            self.chop();
            *c = self.get_current();
        }

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
            return Ok(());
        }

        if c.is_ascii_punctuation() {
            if c == COMMENT {
                let result = self.handle_comment(&mut c, token);
                match result {
                    Ok(()) => return Ok(()),
                    Err(e) => return Err(e)
                }
            };
            
            token.write(c);
            token.token_type = TokenT::NONE__;
            self.chop();
        }

        return Ok(());
    }
    
    pub fn trim_spaces_left(&mut self)  {
        while self.get_current().is_ascii_whitespace()
        {
            self.chop();
        }
    }

    pub fn collect_str(&mut self, token: &mut Token){
        token.token_type = TokenT::VARNAME__;
        let mut c: char = self.get_current();

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
                let mut c: char = self.get_current();
                
                if token.token_type == TokenT::DQUOTE__ {
                    while self.is_not_empty() {
                        if c == DQUOTE {
                            token.token_type = TokenT::STRING__;
                            self.chop();
                            return Ok(token);
                        }

                        token.write(c);
                        self.chop();
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
                
                if c.is_alphanumeric() {
                    self.collect_str(&mut token); // VARNAME__
                    
                    if self.get_current() == OPAR {
                        if token.value == String::from("print") {
                            token.token_type = TokenT::PRINT__;
                            self.chop();
                            return Ok(token);
                        
                        } else {
                            let mut err_text = format!("{}:{}:{} Undefined function {}..", self.file_path, token.loc.row, token.loc.col, token.value);
                            err_text    += &format!("maybe you meant: print?.");
                            return Err(make_error(&err_text));
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
        Ok(Token) => {
            return Token;        
        },
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}



