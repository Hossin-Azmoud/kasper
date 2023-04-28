// ENUMS AND CONSTANTS.

use std::fmt;
use std::collections::HashMap;
/*
#[allow(dead_code, unused_variables)]
pub const UMAX_8_BIT:   u8    = 0xFF;
pub const UMAX_16_BIT:  u16   = 0xFFFF;
pub const UMAX_32_BIT:  u32   = 0xFFFFFFFF;
pub const UMAX_64_BIT:  u64   = 0xFFFFFFFFFFFFFFFF;
pub const UMAX_128_BIT: u128  = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;

pub const MAX_8_BIT:   i8    = 0x7F; // 127
pub const MAX_16_BIT:  i16   = 0x7FFF;
pub const MAX_32_BIT:  i32   = 0x7FFFFFFF;
pub const MAX_64_BIT:  i64   = 0x7FFFFFFFFFFFFFFF;
pub const MAX_128_BIT: i128  = 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF; // >> Division by two.
*/
#[allow(dead_code, unused_variables)]
pub const DQUOTE:         char  = '\"';
pub const SQUOTE:         char  = '\'';
pub const SPACE:          char  = ' ';
pub const NL:             char  = '\n';
pub const TAB:            char  = '\t';
pub const RE:             char  = '\r';
pub const NULLC:           char  = '\0';
pub const OPAR:           char  = '(';
pub const CPAR:           char  = ')';
pub const OCURLY:         char  = '{';
pub const CCURLY:         char  = '}';

pub const PLUS:           char  = '+';
pub const MULT:           char  = '*';
pub const MINUS:          char  = '-';
pub const DIV:            char  = '/';
pub const POW:            char  = '^';

pub const COMA:           char  = ',';
pub const SEMICOLON:      char  = ';';
pub const EQUAL:          char  = '=';
pub const GT:             char  = '>';
pub const LT:             char  = '<';
pub const QM:             char  = '!';
pub const PIPE:           char  = '|';
pub const ESCAPE:         char  = '\\';
pub const THIN_ARROW:     &str  = "->"; 
pub const FAT_ARROW:      &str  = "=>"; 

// Comparison operators.
pub const COMP_EQ:        &str  = "==";
pub const COMP_NOT_EQ:    &str  = "=!";
pub const COMP_LT_EQ:     &str  = "<=";
pub const COMP_GT_EQ:     &str  = ">=";

// TYPLES:
pub const STRING:     &str  = "string"; 
pub const INT:        &str  = "int";
pub const INT_64:     &str  = "i64";
pub const BOOL:       &str  = "bool";

// Key words
pub const WRITE:      &str  = "write";
pub const DEFINE:     &str  = "define";
pub const PROCC:      &str  = "process";
pub const BOOL_TRUE:  &str  = "True";
pub const BOOL_FALSE: &str  = "False";
pub const IF:          &str  = "if";
pub const ELSE:        &str  = "else";

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
#[allow(non_camel_case_types, dead_code)]
pub enum TokenT {
    // Special tokens.
    DIV__,
    COMMENT__,
    POW__,
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
    PIPE__,
    // Other
    NONE__,
    NUMBER__,
    STRING__,
    VARNAME__,
    // Comparison operators
    COMP_EQ__,
    COMP_NOT_EQ__,
    COMP_LT_EQ__,
    COMP_GT_EQ__,


    // primitives
    BOOL_TRUE__,
    BOOL_FALSE__,

    // Types
    INT_T,
    INT_T_64,
    STRING_T,
    BOOL_T,
    
    // Built-ins + funcs..
    FUNC_CALL__,
    WRITE__,
    
    // Key words.
    KEY_WORD_DEFINE__,
    KEY_WORD_PROCESS__,
    ELSE__,
    IF__,
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
            TokenT::DIV__              => "DIV__",
            TokenT::COMMENT__          => "COMMENT__",
            TokenT::WRITE__            => "WRITE__",
            TokenT::KEY_WORD_DEFINE__  => "KEY_WORD_DEFINE__",
            TokenT::KEY_WORD_PROCESS__ => "KEY_WORD_PROCESS__", 
            TokenT::THIN_ARROW__       => "THIN_ARROW__",
            TokenT::FAT_ARROW__        => "FAT_ARROW__",
            TokenT::FUNC_CALL__        => "FUNC_CALL__",
            TokenT::STRING_T           => "STRING_T",
            TokenT::INT_T              => "INT_T",
            TokenT::INT_T_64           => "INT_64",
            TokenT::MULT__             => "MULT__",
            TokenT::BOOL_T             => "BOOL_T",
            TokenT::BOOL_TRUE__        => "BOOL_TRUE__",
            TokenT::BOOL_FALSE__       => "BOOL_FALSE_",
            TokenT::IF__               => "IF__",
            TokenT::ELSE__             => "ELSE__",
            TokenT::COMP_EQ__          => "COMP_EQ__", 
            TokenT::COMP_NOT_EQ__      => "COMP_NOT_EQ__", 
            TokenT::COMP_LT_EQ__       => "COMP_LT_EQ__", 
            TokenT::COMP_GT_EQ__       => "COMP_GT_EQ__", 
            TokenT::PIPE__             => "PIPE__",      
            TokenT::POW__              => "POW__,",
        }; 
        
        return write!(f, "{}", printable)
    }      
}

pub fn make_token_table() -> HashMap<char, TokenT> {
    let mut map: HashMap<char ,TokenT> = HashMap::new();
    
    //Adding all the keys. 
    map.insert(DQUOTE,    TokenT::DQUOTE__);
    map.insert(SQUOTE,    TokenT::SQUOTE__);
    map.insert(OPAR,      TokenT::OPAR__);
    map.insert(CPAR,      TokenT::CPAR__);
    map.insert(OCURLY,    TokenT::OCURLY__);
    map.insert(CCURLY,    TokenT::CCURLY__);
    
    map.insert(PLUS,      TokenT::PLUS__);
    map.insert(MINUS,     TokenT::MINUS__);
    map.insert(MULT,      TokenT::MULT__);
    map.insert(DIV,       TokenT::DIV__);
    map.insert(POW,       TokenT::POW__);
    


    map.insert(COMA,      TokenT::COMA__);
    map.insert(SEMICOLON, TokenT::SEMICOLON__);
    map.insert(EQUAL,     TokenT::EQUAL__);
    map.insert(GT,        TokenT::GT__);
    map.insert(LT,        TokenT::LT__);
    map.insert(QM,        TokenT::QM__);
    map.insert(PIPE,      TokenT::PIPE__);

    // Return the map.
    map
}


pub fn make_prec_table() -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    
    // Adding all the keys.  
    map.insert(POW.to_string(), 4);    
    map.insert(MULT.to_string(), 3);
    map.insert(DIV.to_string(),  3);
    map.insert(PLUS.to_string(), 2);
    map.insert(MINUS.to_string(), 2);

    // Return the map.
    
    map
}
