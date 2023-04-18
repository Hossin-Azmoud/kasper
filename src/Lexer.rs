#[allow(non_snake_case, dead_code)]
use std::fs::File;
use std::io;
use std::io::Read;
use std::fmt;
use std::collections::HashMap;
use std::process::exit;

pub const UMAX_8_BIT:   u8    = (0xFF);
pub const UMAX_16_BIT:  u16   = (0xFFFF);
pub const UMAX_32_BIT:  u32   = (0xFFFFFFFF);
pub const UMAX_64_BIT:  u64   = (0xFFFFFFFFFFFFFFFF);
pub const UMAX_128_BIT: u128  = (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF);

pub const MAX_8_BIT:   i8    = (UMAX_8_BIT   >> 1);
pub const MAX_16_BIT:  i16   = (UMAX_16_BIT  >> 1);
pub const MAX_32_BIT:  i32   = (UMAX_32_BIT  >> 1);
pub const MAX_64_BIT:  i64   = (UMAX_64_BIT  >> 1);
pub const MAX_128_BIT: i128  = (UMAX_128_BIT >> 1); // >> Division by two.

pub const DQUOTE:     char  = '\"';
pub const SQUOTE:     char  = '\'';
pub const SPACE:      char  = ' ';
pub const NL:         char  = '\n';
pub const OPAR:       char  = '(';
pub const CPAR:       char  = ')';
pub const OCURLY:     char  = '{';
pub const CCURLY:     char  = '}';
pub const PLUS:       char  = '+';
pub const MULT:       char  = '*';
pub const MINUS:      char  = '-';
pub const COMA:       char  = ',';
pub const SEMICOLON:  char  = ';';
pub const EQUAL:      char  = '=';
pub const GT:         char  = '>';
pub const LT:         char  = '<';
pub const QM:         char  = '!';
pub const COMMENT:    char  = '/';
pub const ESCAPE:     char  = '\\';
pub const PRINT:      &str  = "print";
pub const DEFINE:     &str  = "define";
pub const PROCC:      &str  = "process";

pub const THIN_ARROW: &str  = "->"; 
pub const FAT_ARROW:  &str  = "=>"; 

// TYPLES:
pub const STRING:     &str  = "string"; 
pub const INT:        &str  = "int";
pub const INT_64:        &str  = "i64";

fn not_implemented(label: &str) {
    println!("{}", label);
}

pub fn make_error(text: &str) -> io::Error { 
    return io::Error::new(io::ErrorKind::Other, text);    
}
// Seperate data from logic.

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
    MULT__,
    COMA__,
    SEMICOLON__,
    EQUAL__,
    GT__,
    LT__,
    QM__,
    THIN_ARROW__,
    FAT_ARROW__,

    // Other
    NONE__,
    NUMBER__,
    STRING__,
    VARNAME__,

    // Types
    INT_T,
    INT_T_64,
    STRING_T,

    // Built-ins + funcs..
    FUNC_CALL__,
    PRINT__,
    
    // Key words.
    KEY_WORD_DEFINE__,
    KEY_WORD_PROCESS__,
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
    map.insert(MULT,TokenT::MULT__);
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
            TokenT::NONE__             => "NONE__",
            TokenT::DQUOTE__           => "DQUOTE__",
            TokenT::SQUOTE__           => "SQUOTE__",
            TokenT::OPAR__             => "OPAR__",
            TokenT::CPAR__             => "CPAR__",
            TokenT::OCURLY__           => "OCURLY__",
            TokenT::CCURLY__           => "CCURLY__",
            TokenT::PLUS__             => "PLUS__",
            TokenT::MINUS__            => "MINUS__",
            TokenT::COMA__             => "COMA__",
            TokenT::SEMICOLON__        => "SEMICOLON__",
            TokenT::EQUAL__            => "EQUAL__",
            TokenT::GT__               => "GT__",
            TokenT::LT__               => "LT__",
            TokenT::NUMBER__           => "NUMBER__",
            TokenT::STRING__           => "STRING__",
            TokenT::QM__               => "QM__",
            TokenT::VARNAME__          => "VARNAME__",
            TokenT::COMMENT__          => "COMMENT__",
            TokenT::PRINT__            => "PRINT__",
            TokenT::KEY_WORD_DEFINE__  => "KEY_WORD_DEFINE__",
            TokenT::KEY_WORD_PROCESS__ => "KEY_WORD_PROCESS__", 
            TokenT::THIN_ARROW__       => "THIN_ARROW__",
            TokenT::FAT_ARROW__        => "FAT_ARROW__",
            TokenT::FUNC_CALL__        => "FUNC_CALL__",
            TokenT::STRING_T           => "STRING_T",
            TokenT::INT_T              => "INT_T",
            TokenT::INT_T_64           => "INT_64",
            TokenT::MULT__             => "MULT__",
        }; 
     
        write!(f, "{}", printable)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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
        
        if self.is_not_empty() {
            
            let mut c = char::from(self.source[self.cur]);

            while c.is_ascii_whitespace() && self.is_not_empty() {
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
            println!("Hi");
            println!("{}", e);
            exit(1);
        }
    }

}
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
    str_map: HashMap<String, String>,
    int_map: HashMap<String, i32>, // 32 bit..
    int_map_64: HashMap<String, i64>, // 32 bit..

}

impl<'a> KasperParser<'a> {
    
     pub fn new(lex: KasperLexer<'a>) -> Self {
        return KasperParser {
            lexer: lex,
            str_map: HashMap::new(),
            int_map: HashMap::new(),
            int_map_64: HashMap::new(),
        };
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

