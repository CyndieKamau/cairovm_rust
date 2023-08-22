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
    //Number(String, Option<String>),
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

        
        //tokenize numbers

        //if ch.is_digit(10) {

            //let number_string: String = characters.by_ref().take_while(|ch| ch.is_digit(10)).collect();


            //let type_hint = if let Some(&ch) = characters.peek() {

                //write code here...

            //};

        //}

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
