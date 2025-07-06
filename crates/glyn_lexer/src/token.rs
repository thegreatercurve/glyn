use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // Keywords
    Keyword(Keyword),

    // Identifiers
    Ident(&'a str),

    // Numeric literals
    Int64(&'a str),
    Float64(&'a str),

    // String literals
    String(&'a str),

    // Punctuators
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    Spread,
    Semicolon,
    Comma,

    // Comparison operators
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    StrictEqual,
    NotEqual,
    StrictNotEqual,

    // Assignment operators
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LogicalAndAssign,
    LogicalOrAssign,
    NullishCoalescingAssign,
    ExponentAssign,

    // Arithmetic operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    // Bitwise operators
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Tilde,

    // Logical operators
    LogicalAnd,
    LogicalOr,
    Not,

    // Shift operators
    LeftShift,
    RightShift,
    UnsignedRightShift,

    // Other operators
    Arrow,
    Question,
    Colon,
    OptionalChaining,
    NullishCoalescing,
    Increment,
    Decrement,

    // Special tokens
    Illegal,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Const,
    If,
    Else,
    Return,
    Function,
    Print,
    True,
    False,
    Null,
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "let" => Ok(Keyword::Let),
            "const" => Ok(Keyword::Const),
            "if" => Ok(Keyword::If),
            "else" => Ok(Keyword::Else),
            "return" => Ok(Keyword::Return),
            "function" => Ok(Keyword::Function),
            "print" => Ok(Keyword::Print),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "null" => Ok(Keyword::Null),
            _ => Err(()),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Keyword::Let => write!(f, "let"),
            Keyword::Const => write!(f, "const"),
            Keyword::If => write!(f, "if"),
            Keyword::Else => write!(f, "else"),
            Keyword::Return => write!(f, "return"),
            Keyword::Function => write!(f, "function"),
            Keyword::Print => write!(f, "print"),
            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),
            Keyword::Null => write!(f, "null"),
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Token::Keyword(kw) => write!(f, "{}", kw),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Int64(value) => write!(f, "{}", value),
            Token::Float64(value) => write!(f, "{}", value),
            Token::String(value) => write!(f, "\"{}\"", value),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Dot => write!(f, "."),
            Token::Spread => write!(f, "..."),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::LessThan => write!(f, "<"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::Equal => write!(f, "=="),
            Token::StrictEqual => write!(f, "==="),
            Token::NotEqual => write!(f, "!="),
            Token::StrictNotEqual => write!(f, "!=="),
            Token::Assign => write!(f, "="),
            Token::PlusAssign => write!(f, "+="),
            Token::MinusAssign => write!(f, "-="),
            Token::MultiplyAssign => write!(f, "*="),
            Token::DivideAssign => write!(f, "/="),
            Token::ModuloAssign => write!(f, "%="),
            Token::LeftShiftAssign => write!(f, "<<="),
            Token::RightShiftAssign => write!(f, ">>="),
            Token::UnsignedRightShiftAssign => write!(f, ">>>="),
            Token::BitwiseAndAssign => write!(f, "&="),
            Token::BitwiseOrAssign => write!(f, "|="),
            Token::BitwiseXorAssign => write!(f, "^="),
            Token::LogicalAndAssign => write!(f, "&&="),
            Token::LogicalOrAssign => write!(f, "||="),
            Token::NullishCoalescingAssign => write!(f, "??="),
            Token::ExponentAssign => write!(f, "**="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::Exponent => write!(f, "**"),
            Token::BitwiseAnd => write!(f, "&"),
            Token::BitwiseOr => write!(f, "|"),
            Token::BitwiseXor => write!(f, "^"),
            Token::Tilde => write!(f, "~"),
            Token::LogicalAnd => write!(f, "&&"),
            Token::LogicalOr => write!(f, "||"),
            Token::Not => write!(f, "!"),
            Token::LeftShift => write!(f, "<<"),
            Token::RightShift => write!(f, ">>"),
            Token::UnsignedRightShift => write!(f, ">>>"),
            Token::Arrow => write!(f, "=>"),
            Token::Question => write!(f, "?"),
            Token::Colon => write!(f, ":"),
            Token::OptionalChaining => write!(f, "?."),
            Token::NullishCoalescing => write!(f, "??"),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}
