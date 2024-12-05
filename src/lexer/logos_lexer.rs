use logos::{Logos, Lexer};

#[derive(Debug, Logos, PartialEq)]
#[logos(extras = usize)]

pub enum Token {

    #[token("as")]
    As,

    #[token("break")]
    Break,

    #[token("bool")]
    Bool,

    #[token("ByteArray")]
    ByteArray,

    #[token("const")]
    Const,

    #[token("continue")]
    Continue,

    #[token("else")]
    Else,

    #[token("enum")]
    Enum,

    #[token("extern")]
    Extern,

    #[token("false")]
    False,

    #[token("fn")]
    Fn,

    #[token("for")]
    For,

    #[token("if")]
    If,

    #[token("impl")]
    Impl,

    #[token("implicits")]
    Implicits,

    #[token("let")]
    Let,

    #[token("loop")]
    Loop,

    #[token("match")]
    Match,

    #[token("mod")]
    Mod,

    #[token("mut")]
    Mut,

    #[token("nopanic")]
    NoPanic,

    #[token("of")]
    Of,

    #[token("pub")]
    Pub,

    #[token("ref")]
    Ref,

    #[token("return")]
    Return,

    #[token("struct")]
    Struct,

    #[token("trait")]
    Trait,

    #[token("true")]
    True,

    #[token("type")]
    Type,

    #[token("fieldElement")]
    Felt252,

    #[token("use")]
    Use,

    #[token("while")]
    While,

    #[token(".")]
    Period,

    #[token(",")]
    Comma,

    #[token("!")]
    Exclamation,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("=")]
    Equals,

    #[token("==")]
    DoubleEquals,

    #[token("!=")]
    NotEquals,

    #[token("*")]
    Asterik,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token("/")]
    Slash,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("<=")]
    LessThanEqual,

    #[token(">=")]
    GreaterThanEqual,

    #[token("%")]
    Percent,

    #[token("&")]
    Ampersand,

    #[token("->")]
    Arrow,

    #[token("(")]
    LeftParenthesis,

    #[token(")")]
    RightParenthesis,

    #[token("{")]
    LeftBrace,

    #[token("}", priority = 2)]
    RightBrace,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("@")]
    Snapshot,

    #[token("#")]
    Hash,

    #[token("||")]
    Or,

    #[token("?")]
    Question,

    #[token("u8")]
    U8,

    #[token("u16")]
    U16,

    #[token("u32")]
    U32,

    #[token("u64")]
    U64,

    #[token("u128")]
    U128,

    #[token("u256")]
    U256,

    #[token("felt252")]
    Felt,

    #[token("usize")]
    Usize,

    #[token(";", priority = 2)]
    SemiColon,

    #[regex(r"'[^']*'", |lex| lex.slice().to_string())] // single-quoted strings
    #[regex(r#""[^"]*""#, |lex| lex.slice().to_string())] // Double-quoted strings
    StringLiteral(String),  // 'Alex', "Alex"

    #[regex(r"'[^']'", |lex| lex.slice().to_string())]
    CharacterLiteral(String), // 'A'

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", callback = process_identifier)]
    Identifier(String),

    //#[regex(r"\d+(_u8|_u16|_u32|_u64|_u128|_u256|_felt252)?", process_number_hint, priority = 2)]
    //#[regex(r"\d+(_(u8|u16|u32|u64|u128|u256|felt252))?", process_number_hint, priority = 2)]
    #[regex(r"\d+(_(u8|u16|u32|u64|u128|u256|felt252))?", process_number_hint, priority = 2)]
    Number(NumberData),

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    // Skip comments
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    //Handling attributes as a single token; TODO:The parser will handle it
    #[regex(r"#\[[^\]]+\]", |lex| lex.slice().to_string())]
    Attribute(String),


    // Error handling,
    //#[regex(r"[^a-zA-Z0-9_ \t\n\f]+", |lex| lex.slice().to_string())]
    //TODO: Tweak the regex to handle grouping of invalid tokens;
    // eg `$invalid` instead of `$` and `invalid`
    //#[regex(r"[^\w \t\n\f;{}\(\)\[\]]+", |lex| lex.slice().to_string())]
    //#[regex(r"[^\w \t\n\f;{}\(\)\[\]]+", |lex| lex.slice().to_string(), priority = 1)] (use this)
    #[regex(r"[^\w \t\n\f;{}\(\)\[\].,:<>=+!]+", |lex| {
        let slice = lex.slice();
        let start = lex.extras; // Access the current offset directly
        let end = start + slice.len(); // Calculate the end of the span
        lex.extras += slice.len(); // Update the offset
        format!(
            "Unexpected character '{}' at span ({}, {})",
            slice, start, end
        )
    }, priority = 1)]
    Error(String),
    

}
#[derive(PartialEq, Debug)]
pub struct NumberData {
    value: String,
    type_hint: Option<String>,
}

impl NumberData {
    pub fn new(value: String, type_hint: Option<String>) -> Self {
        NumberData { value, type_hint }
    }
}

// pub fn validate_number(lex: &mut Lexer<Token>) -> NumberData {
//     let slice = lex.slice();
//     let parts: Vec<&str> = slice.split('_').collect();

//     // The numeric value
//     let value = parts[0].to_string();

//     // Extract type hint, if any
//     let type_hint = if parts.len() > 1 {
//         Some(parts[1].to_string())
//     } else {
//         None
//     };

//     // Validate the type hint
//     let valid_hints = ["u8", "u16", "u32", "u64", "u128", "u256", "felt252"];
//     if let Some(ref hint) = type_hint {
//         if !valid_hints.contains(&hint.as_str()) {
//             panic!("Invalid type hint in number: {}", slice);
//         }
//     }

//     NumberData { value, type_hint }
// }

pub fn process_identifier(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}

// pub fn process_number_hint(lex: &mut Lexer<Token>) -> NumberData {
//     let slice = lex.slice();
//     // Use regex to correctly separate number from optional type hint
//     let re = regex::Regex::new(r"(\d+)(_u8|_u16|_u32|_u64|_u128|_u256|_felt252)?").unwrap();
//     if let Some(captures) = re.captures(slice) {
//         let value = captures.get(1).map_or("", |m| m.as_str()).to_string();
//         let type_hint = captures.get(2).map(|m| m.as_str().trim_start_matches('_').to_string());
//         return NumberData { value, type_hint };
//     }

//     NumberData {
//         value: slice.to_string(),
//         type_hint: None,
//     }
    
// }

pub fn process_number_hint(lex: &mut Lexer<Token>) -> NumberData {
    let slice = lex.slice();
    println!("Processing number hint: {}", slice);
    let parts: Vec<&str> = slice.split('_').collect();

    let value = parts[0].to_string();
    let type_hint = parts.get(1).map(|s| s.to_string());

    NumberData { value, type_hint }
}

// pub fn process_number_hint(lex: &mut Lexer<Token>) -> NumberData {
//     let slice = lex.slice();
//     let parts: Vec<&str> = slice.split('_').collect();

//     let value = parts[0].to_string();
//     let type_hint = parts.get(1).map(|s| s.to_string());

//     // Validate type hint
//     let valid_hints = ["u8", "u16", "u32", "u64", "u128", "u256", "felt252"];
//     if let Some(ref hint) = type_hint {
//         if !valid_hints.contains(&hint.as_str()) {
//             // Reject the token outright by panicking inside the callback
//             panic!("{}", format!("Invalid type hint in number: {}", slice));
//         }
//     }

//     NumberData { value, type_hint }
// }

// pub fn lex_input(input: &str) -> Vec<Token> {
//     let mut lexer = Token::lexer(input);
//     lexer.extras = 0; // Initialize the offset
//     let mut tokens = Vec::new();

//     while let Some(result) = lexer.next() {
//         let slice = lexer.slice(); // Get the current token slice
//         let length = slice.len(); // Calculate the token length

//         println!(
//             "Processing token: '{}' | Start: {} | Length: {} | End: {}",
//             slice,
//             lexer.extras,
//             length,
//             lexer.extras + length
//         );

//         match result {
//             Ok(token) => {
//                 tokens.push(token); // Add the token to the list
//             }
//             Err(_) => {
//                 // Handle errors with detailed span information
//                 let start = lexer.extras;
//                 let end = start + length;
//                 tokens.push(Token::Error(format!(
//                     "Unexpected character '{}' at span ({}, {})",
//                     slice, start, end
//                 )));
//             }
//         }

//         // Fallback: Always increment the offset to account for skipped tokens
//         lexer.extras += length;
//     }

//     tokens
// }

fn calculate_line_column(input: &str, position: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;

    for (idx, char) in input.chars().enumerate() {
        if idx == position {
            break;
        }
        if char == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

pub fn lex_input(input: &str) -> Vec<Token> {
    let mut lexer = Token::lexer(input);
    lexer.extras = 0; // Initialize offset
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        let slice = lexer.slice();
        let length = slice.len();
        let start = lexer.extras;

        match result {
            Ok(token) => tokens.push(token),
            Err(_) => {
                let end = start + length;
                let (line, column) = calculate_line_column(input, start);
                tokens.push(Token::Error(format!(
                    "Unexpected character '{}' at line {}, column {} (span: {}-{})",
                    slice, line, column, start, end
                )));
            }
        }

        lexer.extras += length; // Update offset
    }

    tokens
}
