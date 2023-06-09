use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token(">")]
    Grt,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[regex(r"[0-9]+")]
    ConstInt,

    #[regex(r"([0-9]+)?[.][0-9]+([eE]([-+])?[0-9]+)?")]
    #[regex(r"[0-9]+[eE]([-+])?[0-9]+")]
    ConstFloat,

    #[token("true")]
    #[token("false")]
    ConstBoolean,

    #[regex("\"[^\n\"]*\"")]
    ConstString,

    #[regex(r"[a-zA-Z]+[0-9a-zA-Z]*")]
    Id,

    #[regex("\\s*", logos::skip)]
    #[regex("//.*\n", logos::skip)]
    #[regex("/\\*([^*]|\\*[^/])*\\*/", logos::skip)]
    Skip,



    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
