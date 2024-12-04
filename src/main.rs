use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use logos::Logos;
use crate::lexer::logos_lexer::{Token, lex_input};

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
    let mut lexer = Token::lexer(&input.code); // Create a Logos lexer
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let token_info = match token {
            Ok(t) => TokenInfo {
                token_type: format!("{:?}", t),
                span: (lexer.span().start, lexer.span().end),
                slice: lexer.slice().to_string(),
            },
            Err(_) => TokenInfo {
                token_type: "Error".to_string(),
                span: (lexer.span().start, lexer.span().end),
                slice: lexer.slice().to_string(),
            },
        };
        tokens.push(token_info);
    }

    HttpResponse::Ok().json(TokenOutput { tokens }) // Return JSON response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize the logger

    HttpServer::new(|| {
        let cors = Cors::default()
            //.allowed_origin("http://localhost:3000") // Allow frontend
            .allowed_origin_fn(|origin, _req_head| {
                origin == "http://localhost:3000" || origin == "https://cairovm-rust-starknet.vercel.app/"
            })
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors) // Apply CORS middleware
            .service(web::resource("/tokenize").route(web::post().to(tokenize_code))) // Tokenization endpoint
    })
    .bind("0.0.0.0:8080")? // Bind to localhost
    .run()
    .await
}