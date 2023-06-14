// Token.
use crate::enums::TokenT;

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
    
    pub fn _new(value: String, type_: TokenT) -> Self {
        Token {
            value: value,
            token_type: type_,
            size: 0,
            loc: Location::empty(),
        }
    }
 
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
