// Utility functions.

use std::io;
use std::process::exit;

pub fn not_implemented(label: &str) {
    println!("{}", label);
    exit(0);
}

pub fn make_error(text: &str) -> io::Error { 
    return io::Error::new(io::ErrorKind::Other, text);    
}

pub fn err_check<T>(res: Result<T, io::Error>) -> T {
    match res {
        Ok(v) => {
            return v;
        },
        
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

}

