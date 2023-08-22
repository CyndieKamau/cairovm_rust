use std::io;

#[derive(Debug)]
enum Token {
    //keywords in cairo
    As,
    Break,
    Const,
    Continue,
    Else,
    Enum,
    False,
    Fn,
    If,
    For,
    Let,
    Loop,
    Match,
    Mut,
    Of,
    Return,
    Struct,
    True,

    //Integers in cairo
    //U8,
    //U16,
    //U32,
    //U64,
    //U128,
    //U256,

    //Field element 
    Felt252,

    //Operators and Symbols
    Plus,  //+
    Minus,  //-
    Asterik,  //*
    Percent,  //%
    Exclamation, 
    Comma,  //,
    Arrow,  //->
    SemiColon, //;
    Assign, //=
    Colon, //:
    Equal, //==
    Divide,
    UnderScore,

    //Variables in Cairo

    Identifier(String),
    Number(String, Option<String>),
}

#[derive(Debug)]
enum LexError {

    UnrecognizedToken(String),

}
//Our lexing function

fn lex_input(your_input: &str) -> Result<Vec<Token>, LexError> {

    let mut tokens = Vec::new();

    let mut characters = your_input.chars().peekable();

    while let Some(&ch) = characters.peek() {

        //ignore whitespaces in cairo code

        if ch.is_whitespace() {

            characters.next();
            continue;   

        }

        //tokenize variables 

        if ch.is_ascii_alphabetic() || ch == '_' {


            let ident: String = characters.by_ref().take_while(|&ch| ch.is_ascii_alphanumeric() || ch == '_').collect();

            match ident.as_str() {

                "as" => tokens.push(Token::As),

                "break" => tokens.push(Token::Break),

                "const" => tokens.push(Token::Const),

                "else" => tokens.push(Token::Else),

                "continue" => tokens.push(Token::Continue),

                "enum" => tokens.push(Token::Enum),

                "false" => tokens.push(Token::False),

                "fn" => tokens.push(Token::Fn),

                "if" => tokens.push(Token::If),

                "for" => tokens.push(Token::For),

                "let" => tokens.push(Token::Let),

                "loop" => tokens.push(Token::Loop),

                "match" => tokens.push(Token::Match),

                "mut" => tokens.push(Token::Mut),

                "of" => tokens.push(Token::Of),

                "return" => tokens.push(Token::Return),

                "struct" => tokens.push(Token::Struct),

                "true" => tokens.push(Token::True),

                "felt252" => tokens.push(Token::Felt252),

                _ => tokens.push(Token::Identifier(ident)),

            };

            continue;

        }
        //tokenize numbers

        if ch.is_digit(10) {
            let number_string: String = characters.by_ref().take_while(|ch| ch.is_digit(10)).collect();

            // Start with the assumption that there's no type hint
            let mut type_hint = None;

            // Check for underscore
            if let Some(&'_') = characters.peek() {
                characters.next(); // Consume the underscore

                // You can now branch based on the character after the underscore
                if let Some(&'u') = characters.peek() {
 
                    characters.next();

                    let type_digits: String = characters.by_ref().take_while(|ch| ch.is_digit(10)).collect();
                    
                     match type_digits.as_str() {

                         "8" => {
 
                             println!("Detected hint: u8");
                             type_hint = Some("u8".to_string());
                         },


                         "16" => {

                             println!("Detected hint: u16");
                             type_hint = Some("u16".to_string());
                         },


                         "32" => {

                             println!("Detected hint: u32");
                             type_hint = Some("u32".to_string());
                         },


                         "64" => {

                             println!("Detected hint: u64");
                             type_hint = Some("u64".to_string());
                         },

                         "128" => {

                             println!("Detected hint: u128");
                             type_hint = Some("u128".to_string());
                         },

                         "256" => {

                             println!("Detected hint: 256");
                             type_hint = Some("u256".to_string());
                         }

                         _ => {

                             println!("unexpected token after 'u': u{}", type_digits);
                         }
                     }

                     for _ in type_digits.chars() {

                         characters.next();
                     }

                 } else {
                 
                     let felt_hint: String = characters.by_ref().take_while(|ch| ch.is_ascii_alphabetic()).collect();
                     println!("Detected type hint: {}", felt_hint);

                     if felt_hint == "felt252" {

                         type_hint = Some(felt_hint.clone());

                         for _ in felt_hint.chars() {

                             characters.next();
                         }
                     }
                 
                 }
             }

            // Token creation
            if let Some(type_hint) = type_hint {

                tokens.push(Token::Number(number_string, Some(type_hint)));
            } else {

                tokens.push(Token::Number(number_string, None));
            }

           continue;
        }

        //Check for symbols in our cairo code
        match ch {        
            '-' => {

                characters.next();

                if characters.peek() == Some(&'>') {

                    characters.next();

                    tokens.push(Token::Arrow);

                } else {


                   tokens.push(Token::Minus);
                } 
            },


            '/' => {

               characters.next();

               if characters.peek() == Some(&'/') {

                   characters.next();

                   while let Some(&next_char) = characters.peek() {


                       if next_char == '\n' {


                           break;
                       }

                       characters.next();
                   }

                   continue;
               } else {

                   tokens.push(Token::Divide);
               }

            },


            '=' => {

                characters.next();

                if characters.peek() == Some(&'=') {

                    characters.next();

                    tokens.push(Token::Equal);

                } else {

                    characters.next();

                    tokens.push(Token::Assign);

                }

            },

           

            '+' => { 

                characters.next();
                tokens.push(Token::Plus);

            },

            '*' => { 

                characters.next();
                tokens.push(Token::Asterik);

            },
            
            ',' => {

                characters.next();
                tokens.push(Token::Comma);

            },

            '!' => {

                characters.next();
                tokens.push(Token::Exclamation);

            },

            ';' => {

                characters.next();          
                tokens.push(Token::SemiColon);

            },

            ':' => { 

                characters.next();
                tokens.push(Token::Colon);

            },

            '%' => { 

                characters.next();
                tokens.push(Token::Percent);

            },

            '_' => {

                characters.next();
                tokens.push(Token::UnderScore);
 
            },

            _ => {
                // Here, instead of just skipping, we return an error.
                let unrecognized = characters.by_ref().take_while(|c| !c.is_whitespace()).collect::<String>();
                return Err(LexError::UnrecognizedToken(unrecognized));
            }

         }

    };


    Ok(tokens)

}


fn main() {
    println!("Please enter a simple cairo code here: ");

    let mut your_input = String::new();

    io::stdin().read_line(&mut your_input)

        .expect("Failed to read line");

    //process your cairo code input and get the tokens;

    match lex_input(&your_input) {


    Ok(tokens) => println!("{:?}", tokens),

    Err(LexError::UnrecognizedToken(token)) => eprintln!("Error: Unrecognized token '{}'", token)

    }
}
