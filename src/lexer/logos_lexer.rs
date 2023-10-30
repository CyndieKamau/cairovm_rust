use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] 

enum Token {

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

    



}


fn main() {
    println!("Hello, world!");
}
