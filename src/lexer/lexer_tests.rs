use crate::lexer::logos_lexer;

#[cfg(test)]

mod tests {
    use logos_lexer::*;

    use super::*;

    #[test]
    fn test_keywords() {
        let input = "as break const continue else enum extern false fn for if impl implicits let loop match mod mut";


        let expected_tokens = vec![
            crate::Token::As,
            crate::Token::Break,
            crate::Token::Const,
            crate::Token::Continue,
            crate::Token::Else,
            crate::Token::Enum,
            crate::Token::Extern,
            crate::Token::False,
            crate::Token::Fn,
            crate::Token::For,
            crate::Token::If,
            crate::Token::Impl,
            crate::Token::Implicits,
            crate::Token::Let,
            crate::Token::Loop,
            crate::Token::Match,
            crate::Token::Mod,
            crate::Token::Mut,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_identifiers() {
        let input = "my_variable funding x var123";

        let expected_tokens = vec![
            crate::Token::Identifier("my_variable".to_string()),
            crate::Token::Identifier("funding".to_string()),
            crate::Token::Identifier("x".to_string()),
            crate::Token::Identifier("var123".to_string()),
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_numbers() {
        let input = "48 52_felt252 45_u8 99_u256";

        let expected_tokens = vec![
            crate::Token::Number(NumberData::new("48".to_string(), None)),
            crate::Token::Number(NumberData::new("52".to_string(), Some("felt252".to_string()))),
            crate::Token::Number(NumberData::new("45".to_string(), Some("u8".to_string()))),
            crate::Token::Number(NumberData::new("99".to_string(), Some("u256".to_string()))),
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / % == != < > <= >=";

        let expected_tokens = vec![
            crate::Token::Plus,
            crate::Token::Minus,
            crate::Token::Asterik,
            crate::Token::Slash,
            crate::Token::Percent,
            crate::Token::DoubleEquals,
            crate::Token::NotEquals,
            crate::Token::LessThan,
            crate::Token::GreaterThan,
            crate::Token::LessThanEqual,
            crate::Token::GreaterThanEqual,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_whitespace() {
        let input = "let x = 5;";

        let expected_tokens = vec![
            crate::Token::Let,
            crate::Token::Identifier("x".to_string()),
            crate::Token::Equals,
            crate::Token::Number(NumberData::new("5".to_string(), None)),
            crate::Token::SemiColon,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_symbols_and_delimiters() {
        let input = "( ) { } [ ] , . : :: ; !";

        let expected_tokens = vec![
            crate::Token::LeftParenthesis,
            crate::Token::RightParenthesis,
            crate::Token::LeftBrace,
            crate::Token::RightBrace,
            crate::Token::LeftBracket,
            crate::Token::RightBracket,
            crate::Token::Comma,
            crate::Token::Period,
            crate::Token::Colon,
            crate::Token::DoubleColon,
            crate::Token::SemiColon,
            //crate::Token::UnderScore,
            crate::Token::Exclamation,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_errors() {
        let input = "$invalid @@@";
        let expected_tokens = vec![
            Token::Error("Unexpected character '$' at span (0, 1)".to_string()),  // $ is invalid
            Token::Identifier("invalid".to_string()),
            Token::Error("Unexpected character '@@@' at span (9, 12)".to_string()),  // @@@ is invalid
        ];
        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_mixed_input() {
        let input = "if (x == 5) {let y: u32 = 10} else {let y: u32 = 15};";
        let expected_tokens = vec![
            Token::If,
            Token::LeftParenthesis,
            Token::Identifier("x".to_string()),
            Token::DoubleEquals,
            Token::Number(NumberData::new("5".to_string(), None)),
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Colon,
            Token::U32,
            Token::Equals,
            Token::Number(NumberData::new("10".to_string(), None)),
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Colon,
            Token::U32,
            Token::Equals,
            Token::Number(NumberData::new("15".to_string(), None)),
            Token::RightBrace,
            Token::SemiColon,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_comments() {
        let input = "let x = 5; // This is a comment";
        let expected_tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Number(NumberData::new("5".to_string(), None)),
            Token::SemiColon,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_realistic_program() {
        let input = "
        fn main() {
            let x: u32 = 5;
            let y: felt252 = x + 5_felt252;
            if (y > 20) {
                return y;
            } else {
                x = x - 1;
            }
        }
        ";

        let expected_tokens = vec![
            Token::Fn,
            Token::Identifier("main".to_string()),
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Colon,
            Token::U32,
            Token::Equals,
            Token::Number(NumberData::new("5".to_string(), None)),
            Token::SemiColon,
            Token::Let,
            Token::Identifier("y".to_string()),
            Token::Colon,
            Token::Felt,
            Token::Equals,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Number(NumberData::new("5".to_string(), Some("felt252".to_string()))),
            Token::SemiColon,
            Token::If,
            Token::LeftParenthesis,
            Token::Identifier("y".to_string()),
            Token::GreaterThan,
            Token::Number(NumberData::new("20".to_string(), None)),
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Return,
            Token::Identifier("y".to_string()),
            Token::SemiColon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Identifier("x".to_string()),
            Token::Minus,
            Token::Number(NumberData::new("1".to_string(), None)),
            Token::SemiColon,
            Token::RightBrace,
            Token::RightBrace,
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_handling_attributes() {
        let input1 = "#[test]";
        let input2 = "#[derive(Debug)]";

        let expected_tokens1 = vec![
            Token::Attribute("#[test]".to_string()),
        ];
        let expected_tokens2 = vec![
            Token::Attribute("#[derive(Debug)]".to_string()),
        ];

        assert_eq!(lex_input(input1), expected_tokens1);
        assert_eq!(lex_input(input2), expected_tokens2);
    }

    #[test]
    fn test_underscore() {
        let input = "_";
        let expected_tokens = vec![
            Token::Identifier("_".to_string()),
        ];
        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_combined_underscore_and_identifier() {
        let input = "_ validIdentifier";
        let expected_tokens = vec![
            //Token::UnderScore,
            Token::Identifier("_".to_string()),
            Token::Identifier("validIdentifier".to_string()),
        ];
        assert_eq!(lex_input(input), expected_tokens);
    }


    #[test]
    fn test_invalid_token_sequences() {
        let input = "==+";
        let expected_tokens = vec![
            Token::Error("==+".to_string())
        ];
        assert_eq!(lex_input(input), expected_tokens); // Parser must validate sequences
    }

    //TODO: Try to fix invalid type hints at the parser level
    // TODO: These current tests are failing, I'll have to handle at the parser level 
    #[test]
    fn test_invalid_type_hints() {
        let input = "12_u99";
        let expected_tokens = vec![
            Token::Error("12_u99".to_string()),
        ];

        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_invalid_identifiers() {
        let input = "@variable variable#";
        let expected_tokens = vec![
            //Token::Error("123variable".to_string()),
            Token::Error("@variable".to_string()),
            Token::Error("variable#".to_string()),
        ];
        assert_eq!(lex_input(input), expected_tokens);
    }

    #[test]
    fn test_unterminated_strings() {
        let input = "\"Hello";
        let expected_tokens = vec![
            Token::Error("\"Hello".to_string()), // Lexer does not detect untermination
        ];
        assert_eq!(lex_input(input), expected_tokens); // Parser must detect untermination
    }

    #[test]
    fn test_invalid_number_formats() {
        let input = "42variable 23_";
        let expected_tokens = vec![
            Token::Number(NumberData::new("42".to_string(), None)), // 42
            Token::Identifier("variable".to_string()),             // variable
            Token::Number(NumberData::new("23".to_string(), None)), // 23 (Parser must validate trailing _)
        ];
        assert_eq!(lex_input(input), expected_tokens); // Parser must validate
    }




}