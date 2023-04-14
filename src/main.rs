pub mod lexer;
use crate::lexer::*;
use std::env;
use std::io;


#[warn(unused_variables)]
fn main() -> Result<(), io::Error> {
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
            Ok(())   => continue,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            }
        }
    }

    return Ok(());
}
