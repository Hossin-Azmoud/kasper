pub mod Lexer;

use crate::Lexer::*;

use std::env;
use std::io;

#[allow(unused_variables)]
fn main() -> Result<(), io::Error> {
    // return test_lexer();
    // Command line args
    
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    
    if args.len() < 2 {
        
        println!("---------------------------------");
        println!("The File path was not provided.");
        println!("Usage: {} <path>", program);
        println!("---------------------------------");
        return Ok(());

    }
    
    let src = &args[1];
    // Plugin the parser with the lexer. 
    let lex: KasperLexer = KasperLexer::new(&src);
    let mut parser: KasperParser = KasperParser::new(lex);
    parser.lexer.read()?;
    
    while parser.lexer.is_not_empty() {
       match parser.parse_lexer() {
            Ok(()) => continue,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            }
        }
    }

    return Ok(());
}

#[allow(dead_code)]
fn test_lexer() -> Result<(), io::Error> {
    // Command line args
    let args: Vec<String> = env::args().collect();
    let program = &args[0];
    
    if args.len() < 2 {
        
        println!("---------------------------------");
        println!("The File path was not provided.");
        println!("Usage: {} <path>", program);
        println!("---------------------------------");
        return Ok(());
    }
    
    let src = &args[1];
    // Plugin the parser with the lexer. 
    let mut lex: KasperLexer = KasperLexer::new(&src);
    lex.read()?;
        
    while lex.is_not_empty() {
        let mut nxt = lex.next();
        match nxt {
            Ok(mut token) => {
                token.display_token();
            },
            Err(e) => {
                println!("{}", e);
                return Ok(());
            }
        }
   }
    return Ok(());
}

