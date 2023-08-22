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
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,

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
            let mut type_hint: Option<String> = None;

            // Check for underscore
            if let Some(&'_') = characters.peek() {
                characters.next(); // Consume the underscore

                // You can now branch based on the character after the underscore
                if let Some(&next_ch) = characters.peek() {

                    println!("Character after _: {}", next_ch);
                    match next_ch {
                        'u' => {
                            characters.next();
                            let type_string: String = characters.by_ref().take_while(|ch| ch.is_digit(10)).collect();
                            type_hint = Some(type_string);
                         },
                         'f' => {
                             let felt_hint: String = characters.by_ref().take_while(|ch| ch.is_ascii_alphabetic()).collect();
                             if felt_hint == "felt252" {
                                 type_hint = Some(felt_hint);
                             }
                         }
                         _ => {}
                     }
                 }
             }

            // Now, decide the final type hint, with a default of "felt252" if none was found
            let final_type_hint = type_hint.unwrap_or_else(|| "felt252".to_string());

           // Finally, push the token
           tokens.push(Token::Number(number_string, Some(final_type_hint)));

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

           //continue from here writing matches for integers
            'u' => {

                let number_strings: String = characters.by_ref().take_while(|ch| ch.is_digit(10)).collect();

                match number_strings.as_str() {

                    "8" => tokens.push(Token::U8),

                    "16" => tokens.push(Token::U16),

                    "32" => tokens.push(Token::U32),

                    "64" => tokens.push(Token::U64),

                    "128" => tokens.push(Token::U128),

                    "256" => tokens.push(Token::U256),

                    //add handling of unexpected integers
                    _ => {

                        return Err(LexError::UnrecognizedToken("Unexpected Integer".to_string()));
                    }

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
