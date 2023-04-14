pub mod Lexer;
use crate::Lexer::*;
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
    
    let mut lex: KasperLexer = KasperLexer::new(&src);

    // lex.display();
    lex.read()?;
    let mut token;
    
    while lex.is_not_empty() {
        token = match_lexer_token(lex.next()); 
        if token.token_type != TokenT::COMMENT__ && token.token_type != TokenT::NONE__ {        
           if token.token_type == TokenT::PRINT__ {
                token = match_lexer_token(lex.next());
                if token.token_type == TokenT::STRING__ {
                    let val = token.value.clone();
                    
                    if match_lexer_token(lex.next()).token_type == TokenT::CPAR__ {
                        println!("{}", val);
                        continue;
                    } else {
                        let mut err_text = format!("{}:{}:{} unclosed parent found {} expected )..", lex.file_path, token.loc.row, token.loc.col, token.value);
                        err_text    += &format!("maybe you meant: print?.");
                        println!("{}", err_text);
                        return Ok(());
                    }            
                }             
            }

            token.display_token();
        }
    }

    return Ok(());
}

