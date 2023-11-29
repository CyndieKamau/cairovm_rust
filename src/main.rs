use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;

use logos::Lexer;
use crate::lexer::Token;
mod lexer;


#[derive(Deserialize)]
struct CodeInput {
    code: String,
}

#[derive(Serialize)]
struct TokenInfo {
    token_type: String,   // Representing the type of the token (e.g., "Let", "Identifier", etc.)
    span: (usize, usize), // Tuple representing the start and end positions
    slice: String,        // The slice of the source code corresponding to this token
}

#[derive(Serialize)]
struct TokenOutput {
    tokens: Vec<TokenInfo>,
}

async fn tokenize_code(input: web::Json<CodeInput>) -> impl Responder {
    let mut lexer = Lexer::<Token>::new(&input.code);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                let token_info = TokenInfo {
                    token_type: format!("{:?}", token), // Adjust based on how you want to represent the token type
                    span: (lexer.span().start, lexer.span().end),
                    slice: lexer.slice().to_string(),
                };
                tokens.push(token_info);
            },
            Err(_) => {
                // Handle lexical errors here
                // For now, we'll just log to console and continue
                eprintln!("Lexical error at span: {:?}", lexer.span());
            }
        }
    }

    HttpResponse::Ok().json(TokenOutput { tokens })
}

//Setting up the Actix web server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();  //Initialize the logger

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/tokenize").route(web::post().to(tokenize_code)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
