use std::fmt::{Display, Error, Formatter};

// 12.7.2 Keywords and Reserved Words
// https://262.ecma-international.org/16.0/#sec-keywords-and-reserved-words
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Keyword {
    Await,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
    Yield,

    // Strict mode future reserved words.
    Let,
    Static,
    Implements,
    Interface,
    Package,
    Private,
    Protected,
    Public,

    // Appear as keywords within certain syntactic productions, at places where Identifier is not allowed.
    As,
    Async,
    From,
    Get,
    Of,
    Set,
    Target,

    // Utility
    Print,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Keyword::Await => write!(f, "await"),
            Keyword::Break => write!(f, "break"),
            Keyword::Case => write!(f, "case"),
            Keyword::Catch => write!(f, "catch"),
            Keyword::Class => write!(f, "class"),
            Keyword::Const => write!(f, "const"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Debugger => write!(f, "debugger"),
            Keyword::Default => write!(f, "default"),
            Keyword::Delete => write!(f, "delete"),
            Keyword::Do => write!(f, "do"),
            Keyword::Else => write!(f, "else"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::Export => write!(f, "export"),
            Keyword::Extends => write!(f, "extends"),
            Keyword::False => write!(f, "false"),
            Keyword::Finally => write!(f, "finally"),
            Keyword::For => write!(f, "for"),
            Keyword::Function => write!(f, "function"),
            Keyword::If => write!(f, "if"),
            Keyword::Import => write!(f, "import"),
            Keyword::In => write!(f, "in"),
            Keyword::Instanceof => write!(f, "instanceof"),
            Keyword::New => write!(f, "new"),
            Keyword::Null => write!(f, "null"),
            Keyword::Return => write!(f, "return"),
            Keyword::Super => write!(f, "super"),
            Keyword::Switch => write!(f, "switch"),
            Keyword::This => write!(f, "this"),
            Keyword::Throw => write!(f, "throw"),
            Keyword::True => write!(f, "true"),
            Keyword::Try => write!(f, "try"),
            Keyword::Typeof => write!(f, "typeof"),
            Keyword::Var => write!(f, "var"),
            Keyword::Void => write!(f, "void"),
            Keyword::While => write!(f, "while"),
            Keyword::With => write!(f, "with"),
            Keyword::Yield => write!(f, "yield"),
            Keyword::Let => write!(f, "let"),
            Keyword::Static => write!(f, "static"),
            Keyword::Implements => write!(f, "implements"),
            Keyword::Interface => write!(f, "interface"),
            Keyword::Package => write!(f, "package"),
            Keyword::Private => write!(f, "private"),
            Keyword::Protected => write!(f, "protected"),
            Keyword::Public => write!(f, "public"),
            Keyword::As => write!(f, "as"),
            Keyword::Async => write!(f, "async"),
            Keyword::From => write!(f, "from"),
            Keyword::Get => write!(f, "get"),
            Keyword::Of => write!(f, "of"),
            Keyword::Set => write!(f, "set"),
            Keyword::Target => write!(f, "target"),
            Keyword::Print => write!(f, "print"),
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "await" => Ok(Keyword::Await),
            "break" => Ok(Keyword::Break),
            "case" => Ok(Keyword::Case),
            "catch" => Ok(Keyword::Catch),
            "class" => Ok(Keyword::Class),
            "const" => Ok(Keyword::Const),
            "continue" => Ok(Keyword::Continue),
            "debugger" => Ok(Keyword::Debugger),
            "default" => Ok(Keyword::Default),
            "delete" => Ok(Keyword::Delete),
            "do" => Ok(Keyword::Do),
            "else" => Ok(Keyword::Else),
            "enum" => Ok(Keyword::Enum),
            "export" => Ok(Keyword::Export),
            "extends" => Ok(Keyword::Extends),
            "false" => Ok(Keyword::False),
            "finally" => Ok(Keyword::Finally),
            "for" => Ok(Keyword::For),
            "function" => Ok(Keyword::Function),
            "if" => Ok(Keyword::If),
            "import" => Ok(Keyword::Import),
            "in" => Ok(Keyword::In),
            "instanceof" => Ok(Keyword::Instanceof),
            "new" => Ok(Keyword::New),
            "null" => Ok(Keyword::Null),
            "return" => Ok(Keyword::Return),
            "super" => Ok(Keyword::Super),
            "switch" => Ok(Keyword::Switch),
            "this" => Ok(Keyword::This),
            "throw" => Ok(Keyword::Throw),
            "true" => Ok(Keyword::True),
            "try" => Ok(Keyword::Try),
            "typeof" => Ok(Keyword::Typeof),
            "var" => Ok(Keyword::Var),
            "void" => Ok(Keyword::Void),
            "while" => Ok(Keyword::While),
            "with" => Ok(Keyword::With),
            "yield" => Ok(Keyword::Yield),
            "let" => Ok(Keyword::Let),
            "static" => Ok(Keyword::Static),
            "implements" => Ok(Keyword::Implements),
            "interface" => Ok(Keyword::Interface),
            "package" => Ok(Keyword::Package),
            "private" => Ok(Keyword::Private),
            "protected" => Ok(Keyword::Protected),
            "public" => Ok(Keyword::Public),
            "as" => Ok(Keyword::As),
            "async" => Ok(Keyword::Async),
            "from" => Ok(Keyword::From),
            "get" => Ok(Keyword::Get),
            "of" => Ok(Keyword::Of),
            "set" => Ok(Keyword::Set),
            "target" => Ok(Keyword::Target),
            "print" => Ok(Keyword::Print),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token<'a> {
    // Keywords or Identifiers
    Keyword(Keyword),
    Ident(&'a str),
    PrivateIdentifier(&'a str),

    // Literals
    String(&'a str),
    Int64(&'a str),
    Float64(&'a str),
    BigIntLiteral(&'a str),
    RegularExpressionLiteral(&'a str),

    // Punctuators
    OptionalChaining,
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
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Increment,
    Decrement,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    BitAnd,
    BitOr,
    BitXor,
    Not,
    Tilde,
    LogicalAnd,
    LogicalOr,
    NullishCoalescing,
    Question,
    Colon,
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    ModuloAssign,
    ExponentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    LogicalAndAssign,
    LogicalOrAssign,
    NullishCoalescingAssign,
    Arrow,
    DivideAssign,

    // Template Literals
    TemplateNoSubstitution,
    TemplateHead,
    TemplateMiddle,
    TemplateTail,

    // Utility
    Illegal,
    Eof,
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(s: &'a str) -> Self {
        match Keyword::try_from(s) {
            Ok(keyword) => Token::Keyword(keyword),
            Err(_) => Token::Ident(s),
        }
    }
}

impl<'a> Token<'a> {
    // 12.7 Names and Keywords
    // https://262.ecma-international.org/16.0/#prod-PrivateIdentifier
    pub(crate) fn is_private_identifier(&self) -> bool {
        matches!(self, Token::PrivateIdentifier(_))
    }

    pub(crate) fn is_identifier_name(&self) -> bool {
        matches!(self, Token::Ident(_) | Token::Keyword(_))
    }

    // 12.9.6 Template Literal Lexical Components
    // https://262.ecma-international.org/16.0/#sec-template-literal-lexical-components
    pub(crate) fn is_template_start(&self) -> bool {
        matches!(self, Token::TemplateNoSubstitution | Token::TemplateHead)
    }

    pub(crate) fn is_template_part(&self) -> bool {
        matches!(
            self,
            Token::TemplateNoSubstitution
                | Token::TemplateHead
                | Token::TemplateMiddle
                | Token::TemplateTail
        )
    }

    // 13.1 Identifiers
    // https://262.ecma-international.org/16.0/#prod-IdentifierReference
    pub(crate) fn is_identifier_reference(&self) -> bool {
        matches!(self, Token::Keyword(Keyword::Yield | Keyword::Await)) || self.is_identifier()
    }

    // https://262.ecma-international.org/16.0/#prod-ReservedWord
    pub(crate) fn is_reserved_keyword(&self) -> bool {
        matches!(
            self,
            Token::Keyword(
                Keyword::Await
                    | Keyword::Break
                    | Keyword::Case
                    | Keyword::Catch
                    | Keyword::Class
                    | Keyword::Const
                    | Keyword::Continue
                    | Keyword::Debugger
                    | Keyword::Default
                    | Keyword::Delete
                    | Keyword::Do
                    | Keyword::Else
                    | Keyword::Enum
                    | Keyword::Export
                    | Keyword::Extends
                    | Keyword::False
                    | Keyword::Finally
                    | Keyword::For
                    | Keyword::Function
                    | Keyword::If
                    | Keyword::Import
                    | Keyword::In
                    | Keyword::Instanceof
                    | Keyword::New
                    | Keyword::Null
                    | Keyword::Return
                    | Keyword::Super
                    | Keyword::Switch
                    | Keyword::This
                    | Keyword::Throw
                    | Keyword::True
                    | Keyword::Try
                    | Keyword::Typeof
                    | Keyword::Var
                    | Keyword::Void
                    | Keyword::While
                    | Keyword::With
                    | Keyword::Yield
            )
        )
    }

    // 13.1 Identifiers
    // https://262.ecma-international.org/16.0/#prod-BindingIdentifier
    pub(crate) fn is_binding_identifier(&self) -> bool {
        matches!(self, Token::Keyword(Keyword::Yield | Keyword::Await)) || self.is_identifier()
    }

    // https://262.ecma-international.org/16.0/#prod-Identifier
    pub(crate) fn is_identifier(&self) -> bool {
        self.is_identifier_name() && !self.is_reserved_keyword()
    }

    // 13.2.5 Property Accessors
    // https://262.ecma-international.org/16.0/#prod-PropertyName
    pub(crate) fn is_property_name(&self) -> bool {
        matches!(
            self,
            Token::String(_) | Token::Int64(_) | Token::BigIntLiteral(_)
        ) || self.is_identifier_name()
    }

    // 13.4 Update Expressions
    // https://262.ecma-international.org/16.0/#prod-UpdateExpression
    pub(crate) fn is_update_operator(&self) -> bool {
        matches!(self, Token::Increment | Token::Decrement)
    }

    //  13.5 Unary Operators
    // https://262.ecma-international.org/16.0/#prod-UnaryExpression
    pub(crate) fn is_unary_operator(&self) -> bool {
        matches!(
            self,
            Token::Keyword(Keyword::Delete | Keyword::Void | Keyword::Typeof)
                | Token::Plus
                | Token::Minus
                | Token::Not
                | Token::Tilde
        )
    }

    // 13.6 Exponentiation Operators
    // https://262.ecma-international.org/16.0/#prod-ExponentiationExpression

    // 13.7 Multiplicative Operators
    // https://262.ecma-international.org/16.0/#prod-MultiplicativeExpression

    // 13.8 Additive Operators
    // https://262.ecma-international.org/16.0/#prod-AdditiveExpression

    // 13.9 Bitwise Shift Operators
    // https://262.ecma-international.org/16.0/#prod-ShiftExpression

    // 13.10 Relational Operators
    // https://262.ecma-international.org/16.0/#prod-RelationalExpression

    // 13.11 Equality Operators
    // https://262.ecma-international.org/16.0/#prod-EqualityExpression

    // 13.12 Binary Bitwise Operators
    // https://262.ecma-international.org/16.0/#prod-BitwiseANDExpression
    // https://262.ecma-international.org/16.0/#prod-BitwiseXORExpression
    // https://262.ecma-international.org/16.0/#prod-BitwiseORExpression

    // 13.13 Binary Logical Operators
    // https://262.ecma-international.org/16.0/#prod-LogicalANDExpression
    // https://262.ecma-international.org/16.0/#prod-LogicalORExpression
    pub(crate) fn is_binary_operator(&self) -> bool {
        matches!(self, |Token::BitOr| Token::BitXor
            | Token::BitAnd
            | Token::Equal
            | Token::NotEqual
            | Token::StrictEqual
            | Token::StrictNotEqual
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual
            | Token::Keyword(Keyword::Instanceof | Keyword::In)
            | Token::LeftShift
            | Token::RightShift
            | Token::UnsignedRightShift
            | Token::Plus
            | Token::Minus
            | Token::Multiply
            | Token::Divide
            | Token::Modulo
            | Token::Exponent)
    }

    // 13.13 Binary Logical Operators
    // https://262.ecma-international.org/16.0/#prod-LogicalORExpression
    pub(crate) fn is_logical_operator(&self) -> bool {
        matches!(
            self,
            Token::NullishCoalescing | Token::LogicalAnd | Token::LogicalOr
        )
    }

    // 13.15 Assignment Operators
    // https://262.ecma-international.org/16.0/#prod-AssignmentOperator
    pub(crate) fn is_assignment_operator(&self) -> bool {
        matches!(self, |Token::MultiplyAssign| Token::DivideAssign
            | Token::ModuloAssign
            | Token::PlusAssign
            | Token::MinusAssign
            | Token::LeftShiftAssign
            | Token::RightShiftAssign
            | Token::UnsignedRightShiftAssign
            | Token::BitAndAssign
            | Token::BitOrAssign
            | Token::BitXorAssign
            | Token::ExponentAssign
            | Token::Assign
            // ES2021
            | Token::LogicalAndAssign
            | Token::LogicalOrAssign
            | Token::NullishCoalescingAssign)
    }

    // 13.15.5 Destructuring Assignment
    // https://262.ecma-international.org/16.0/#prod-DestructuringAssignment
    pub(crate) fn is_assignment_pattern_start(&self) -> bool {
        matches!(self, Token::LeftBracket | Token::LeftBrace)
    }

    // 14 ECMAScript Language: Statements and Declarations
    // https://262.ecma-international.org/16.0/#prod-Declaration
    pub(crate) fn is_declaration_start(&self) -> bool {
        matches!(self, Token::Keyword(Keyword::Function | Keyword::Class))
            | self.is_lexical_declaration_start()
    }

    // 14 ECMAScript Language: Statements and Declarations
    // https://262.ecma-international.org/16.0/#prod-HoistableDeclaration
    pub(crate) fn is_hoistable_declaration_start(&self) -> bool {
        matches!(self, Token::Keyword(Keyword::Function | Keyword::Async))
    }

    // 14.3.1 Let and Const Declarations
    // https://262.ecma-international.org/16.0/#prod-LexicalDeclaration
    pub(crate) fn is_lexical_declaration_start(&self) -> bool {
        matches!(
            self,
            Token::Keyword(Keyword::Let) | Token::Keyword(Keyword::Const)
        )
    }

    // 14.3.1 Let and Const Declarations
    // https://262.ecma-international.org/16.0/#prod-LexicalBinding
    pub(crate) fn is_lexical_binding_start(&self) -> bool {
        self.is_binding_identifier() || self.is_binding_pattern_start()
    }

    // 14.3.2 Variable Statement
    // https://262.ecma-international.org/16.0/#prod-VariableStatement
    pub(crate) fn is_variable_declaration_start(&self) -> bool {
        matches!(self, Token::Keyword(Keyword::Var))
    }

    // 14.3.3 Destructuring Binding Patterns
    // https://262.ecma-international.org/16.0/#prod-BindingPattern
    pub(crate) fn is_binding_pattern_start(&self) -> bool {
        matches!(self, Token::LeftBracket | Token::LeftBrace)
    }

    // 15.7 Class Definitions
    // https://262.ecma-international.org/16.0/#prod-ClassElementName
    pub(crate) fn is_class_declaration_start(&self) -> bool {
        matches!(self, Token::Keyword(Keyword::Class))
    }

    pub(crate) fn is_class_element_name(&self) -> bool {
        matches!(self, Token::PrivateIdentifier(_)) || self.is_property_name()
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            // Keywords or Identifiers
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::PrivateIdentifier(ident) => write!(f, "{}", ident),

            // Literals
            Token::String(value) => write!(f, "{}", value),
            Token::Int64(value) => write!(f, "{}", value),
            Token::Float64(value) => write!(f, "{}", value),
            Token::BigIntLiteral(value) => write!(f, "{}", value),
            Token::RegularExpressionLiteral(value) => write!(f, "{}", value),

            // Punctuators
            Token::OptionalChaining => write!(f, "?."),
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
            Token::GreaterThan => write!(f, ">"),
            Token::LessThanEqual => write!(f, "<="),
            Token::GreaterThanEqual => write!(f, ">="),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::StrictEqual => write!(f, "==="),
            Token::StrictNotEqual => write!(f, "!=="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::Exponent => write!(f, "**"),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            Token::LeftShift => write!(f, "<<"),
            Token::RightShift => write!(f, ">>"),
            Token::UnsignedRightShift => write!(f, ">>>"),
            Token::BitAnd => write!(f, "&"),
            Token::BitOr => write!(f, "|"),
            Token::BitXor => write!(f, "^"),
            Token::Not => write!(f, "!"),
            Token::Tilde => write!(f, "~"),
            Token::LogicalAnd => write!(f, "&&"),
            Token::LogicalOr => write!(f, "||"),
            Token::NullishCoalescing => write!(f, "??"),
            Token::Question => write!(f, "?"),
            Token::Colon => write!(f, ":"),
            Token::Assign => write!(f, "="),
            Token::PlusAssign => write!(f, "+="),
            Token::MinusAssign => write!(f, "-="),
            Token::MultiplyAssign => write!(f, "*="),
            Token::ModuloAssign => write!(f, "%="),
            Token::ExponentAssign => write!(f, "**="),
            Token::LeftShiftAssign => write!(f, "<<="),
            Token::RightShiftAssign => write!(f, ">>="),
            Token::UnsignedRightShiftAssign => write!(f, ">>>="),
            Token::BitAndAssign => write!(f, "&="),
            Token::BitOrAssign => write!(f, "|="),
            Token::BitXorAssign => write!(f, "^="),
            Token::LogicalAndAssign => write!(f, "&&="),
            Token::LogicalOrAssign => write!(f, "||="),
            Token::NullishCoalescingAssign => write!(f, "??="),
            Token::Arrow => write!(f, "=>"),
            Token::DivideAssign => write!(f, "/="),

            // Template Literals
            Token::TemplateNoSubstitution => write!(f, "`"),
            Token::TemplateHead => write!(f, "`"),
            Token::TemplateMiddle => write!(f, "${{"),
            Token::TemplateTail => write!(f, "`"),

            // Utility
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_precedence#table
#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum BinOpPrecedence {
    Lowest,
    Comma,
    Spread,
    Yield,
    Assignment,
    Conditional,
    Coalesce,
    LogicalOR,
    LogicalAND,
    BitOR,
    BitXOR,
    BitAND,
    Equality,
    Relational,
    Shift,
    Additive,
    Multiplicative,
    Exponentiation,
    Unary,
    Update,
    LeftHandSide,
    OptionalChain,
    Member,
    Primary,
    Parentheses,
}

impl BinOpPrecedence {
    pub(crate) fn is_right_associative(&self) -> bool {
        matches!(
            self,
            BinOpPrecedence::Exponentiation | BinOpPrecedence::Assignment
        )
    }
}

impl<'a> From<Token<'a>> for BinOpPrecedence {
    fn from(token: Token<'a>) -> Self {
        match token {
            Token::NullishCoalescing => BinOpPrecedence::Coalesce,
            Token::LogicalOr => BinOpPrecedence::LogicalOR,
            Token::LogicalAnd => BinOpPrecedence::LogicalAND,
            Token::BitOr => BinOpPrecedence::BitOR,
            Token::BitXor => BinOpPrecedence::BitXOR,
            Token::BitAnd => BinOpPrecedence::BitAND,
            Token::Equal | Token::NotEqual | Token::StrictEqual | Token::StrictNotEqual => {
                BinOpPrecedence::Equality
            }
            Token::LessThan
            | Token::GreaterThan
            | Token::LessThanEqual
            | Token::GreaterThanEqual => BinOpPrecedence::Relational,
            Token::Keyword(Keyword::Instanceof) | Token::Keyword(Keyword::In) => {
                BinOpPrecedence::Relational
            }
            Token::LeftShift | Token::RightShift | Token::UnsignedRightShift => {
                BinOpPrecedence::Shift
            }
            Token::Plus | Token::Minus => BinOpPrecedence::Additive,
            Token::Multiply | Token::Divide | Token::Modulo => BinOpPrecedence::Multiplicative,
            Token::Exponent => BinOpPrecedence::Exponentiation,
            Token::Comma => BinOpPrecedence::Comma,
            _ => BinOpPrecedence::Lowest,
        }
    }
}
