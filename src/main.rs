mod asst;

use asst::lexer::Scanner;
use std::io::prelude::*;
use std::fs::File;

use crate::asst::lexer::Token;
//Why tf is string not getting recongnized
fn main() -> std::io::Result<()> { 
    let path : &str = "D:/dev/c_compiler/src/input/cFile.txt";
    let mut file = File::open(path)?;
    let mut buffer : String = String::new();
    file.read_to_string(&mut buffer)?;
    let mut tokens : Vec<Token> = Vec::new();
    let mut scanner = Scanner::new(buffer,&mut tokens);
    scanner.scan_tokens();

    for token in &tokens {
        println!("{:?}",token);
    }

    Ok(())

}
