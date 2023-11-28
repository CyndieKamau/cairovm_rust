use logos::Lexer;
use crate::lexer::Token;
mod lexer;


fn main() {
    // Replace this with the actual source code you want to tokenize
    let input = "let mut x: u32 = 5;"; 

    let mut lexer = Lexer::<Token>::new(input);

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                // Process the token, for example, print it
                println!("{:?} (Span: {:?}, Slice: '{}')", token, lexer.span(), lexer.slice());
            },
            Err(_) => {
                // Handle lexical errors here
                eprintln!("Lexical error at span: {:?}", lexer.span());
            }
        }
    }
}