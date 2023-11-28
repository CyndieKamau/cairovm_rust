use logos::{Logos, Lexer};

#[derive(Debug, Logos, PartialEq)]

pub enum Token {

    #[token("as")]
    As,

    #[token("break")]
    Break,

    #[token("const")]
    Const,

    #[token("continue")]
    Continue,

    #[token("else")]
    Else,

    #[token("enum")]
    Enum,

    #[token("false")]
    False,

    #[token("fn")]
    Fn,

    #[token("if")]
    If,

    #[token("for")]
    For,

    #[token("let")]
    Let,

    #[token("loop")]
    Loop,

    #[token("match")]
    Match,

    #[token("mut")]
    Mut,

    #[token("of")]
    Of,

    #[token("return")]
    Return,

    #[token("struct")]
    Struct,

    #[token("true")]
    True,

    #[token("trait")]
    Trait,

    #[token("fieldElement")]
    Felt252,

    #[token(".")]
    Period,

    #[token("!")]
    Exclamation,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("=")]
    Assign,

    #[token("==")]
    Equal,

    #[token(";")]
    SemiColon,

    #[token("*")]
    Asterik,

    #[token(":")]
    Colon,

    #[token("_")]
    UnderScore,

    #[token("/")]
    Divide,

    #[token("%")]
    Percent,

    #[token("&")]
    Ampersand,

    #[token("->")]
    Arrow,

    #[token("()")]
    Parenthesis,

    #[token("{}")]
    Braces,

    #[token("[]")]
    Bracket,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", callback = process_identifier)]
    Identifier(String),

    #[regex(r"\d+(_u8|_u16|_u32|_u64|_u128|_u256|_felt252)?", process_number_hint)]
    Number(NumberData),

    // Error handling,
    Error,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

}
#[derive(PartialEq, Debug)]
pub struct NumberData {
    value: String,
    type_hint: Option<String>,
}

pub fn process_identifier(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}

pub fn process_number_hint(lex: &mut Lexer<Token>) -> NumberData {
    let slice = lex.slice();
    let parts: Vec<&str> = slice.split('_').collect();
    NumberData {
        value: parts[0].to_string(),
        type_hint: parts.get(1).map(|s| s.to_string()),
    }
}

pub fn lex_input(input: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => tokens.push(token),
            Err(_) => {
                // Here you can handle the error, e.g., log it or push a custom error token
                // For now, we'll just push a generic Error token
                tokens.push(Token::Error);
            }
        }
    }

    tokens
}



