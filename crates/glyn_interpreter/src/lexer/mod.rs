mod tests;
mod token;

use core::fmt;

use glyn_unicode::{is_unicode_id_continue, is_unicode_id_start};

pub(crate) use token::{BinOpPrecedence, Keyword, Token};

#[derive(Debug)]
pub(crate) enum LexerError {
    UnexpectedChar,
    InvalidStringToKeywordConversion,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar => {
                write!(f, "Unexpected character in the input string.")
            }
            LexerError::InvalidStringToKeywordConversion => {
                write!(f, "Unexpected attempt to convert a string to a keyword.")
            }
        }
    }
}

// 12.1 Unicode Format-Control Characters
// https://262.ecma-international.org/16.0/#sec-unicode-format-control-characters
// const ZWNJ: char = '\u{200C}'; // Used in IdentifierPart
// const ZWJ: char = '\u{200D}'; // Used in IdentifierPart

// 12.2 White Space
// https://262.ecma-international.org/16.0/#sec-white-space
fn is_char_whitespace(ch: char) -> bool {
    matches!(
        ch,
        '\u{0009}' // CHARACTER TABULATION
        | '\u{000B}' // LINE TABULATION
        | '\u{000C}' // FORM FEED (FF)
        | '\u{0020}' // SPACE
        | '\u{00A0}' // NO-BREAK SPACE	
        | '\u{1680}' // OGHAM SPACE MARK	
        | '\u{2000}' // EN QUAD	
        | '\u{2001}' // EM QUAD	
        | '\u{2002}' // EN SPACE	
        | '\u{2003}' // EM SPACE	
        | '\u{2004}' // THREE-PER-EM SPACE	
        | '\u{2005}' // FOUR-PER-EM SPACE	
        | '\u{2006}' // SIX-PER-EM SPACE	
        | '\u{2007}' // FIGURE SPACE	
        | '\u{2008}' // PUNCTUATION SPACE	
        | '\u{2009}' // THIN SPACE	
        | '\u{200A}' // HAIR SPACE	
        | '\u{202F}' // NARROW NO-BREAK SPACE	
        | '\u{205F}' // MEDIUM MATHEMATICAL SPAC
        | '\u{3000}' // IDEOGRAPHIC SPACE
    )
}

// 12.3 Line Terminators
// https://262.ecma-international.org/16.0/#sec-line-terminators
fn is_char_line_terminator(ch: char) -> bool {
    matches!(
        ch,
        '\u{000A}' // LINE FEED (LF)
        | '\u{000D}' // CARRIAGE RETURN (CR)
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

// 12.7 Names and Keywords
// https://262.ecma-international.org/16.0/#sec-names-and-keywords
fn is_char_identifier_start_simple(ch: char) -> bool {
    matches!(ch, '$' | '_') || ch.is_ascii_alphabetic()
}

fn is_char_identifier_start(ch: char) -> bool {
    is_char_identifier_start_simple(ch) || is_unicode_id_start(ch)
}

fn is_char_identifier_part_simple(ch: char) -> bool {
    matches!(ch, '$' | '_') || ch.is_ascii_alphanumeric()
}

fn is_char_identifier_part(ch: char) -> bool {
    is_char_identifier_part_simple(ch) || is_unicode_id_continue(ch)
}

// 12.8 Punctuators
// https://262.ecma-international.org/16.0/#sec-punctuators
fn is_char_punctuator_start(ch: char) -> bool {
    matches!(
        ch,
        '{' | '('
            | ')'
            | '['
            | ']'
            | '.'
            | ';'
            | ','
            | '<'
            | '>'
            | '='
            | '!'
            | '+'
            | '-'
            | '*'
            | '%'
            | '&'
            | '|'
            | '^'
            | '~'
            | '?'
            | ':'
            | '/'
            | '}'
    )
}

pub(crate) struct Lexer<'a> {
    source: &'a str,
    chars: Vec<(usize, char)>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            source: input,
            chars: input.char_indices().collect(),
            pos: 0,
        }
    }

    fn error<T>(&self, error_type: LexerError) -> Result<T, LexerError> {
        Err(error_type)
    }

    fn current(&self) -> char {
        self.chars[self.pos].1
    }

    fn current_byte_pos(&self) -> usize {
        if self.is_eof() {
            return self.source.len();
        }

        self.chars[self.pos].0
    }

    fn source_str(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn advance_if(&mut self, ch: char) -> bool {
        if !self.is_eof() && self.current() == ch {
            self.advance();

            true
        } else {
            false
        }
    }

    fn advance_if_2(&mut self, ch_1: char, ch_2: char) -> bool {
        if !self.is_eof() && self.current() == ch_1 && self.peek_char(1) == ch_2 {
            self.advance();
            self.advance();

            true
        } else {
            false
        }
    }

    fn advance_if_3(&mut self, ch_1: char, ch_2: char, ch_3: char) -> bool {
        if !self.is_eof()
            && self.current() == ch_1
            && self.peek_char(1) == ch_2
            && self.peek_char(2) == ch_3
        {
            self.advance();
            self.advance();
            self.advance();

            true
        } else {
            false
        }
    }

    fn peek_char(&self, n_chars: usize) -> char {
        self.chars[self.pos + n_chars].1
    }

    // 12.2 White Space
    // https://262.ecma-international.org/16.0/#sec-white-space

    // 12.3 Line Terminators
    // https://262.ecma-international.org/16.0/#sec-line-terminators
    fn js_skip_whitespace_and_line_terminators(&mut self) {
        while !self.is_eof() {
            let ch = self.current();

            if is_char_whitespace(ch) || is_char_line_terminator(ch) {
                self.advance();
            } else {
                break;
            }
        }
    }

    // 12.7 Names and Keywords
    // https://262.ecma-international.org/16.0/#sec-names-and-keywords
    fn js_lex_identifier_name_or_keyword(&mut self) -> Result<Token<'a>, LexerError> {
        let start = self.current_byte_pos();

        self.js_read_identifier_to_end()?;

        let str_value = self.source_str(start, self.current_byte_pos());

        match Keyword::try_from(str_value).ok() {
            Some(keyword) => Ok(Token::Keyword(keyword)),
            None => Ok(Token::Ident(str_value)),
        }
    }

    fn js_read_identifier_to_end(&mut self) -> Result<(), LexerError> {
        if is_char_identifier_start(self.current()) {
            self.advance();

            while !self.is_eof() && is_char_identifier_part(self.current()) {
                self.advance();
            }
        }

        Ok(())
    }

    // 12.8 Punctuators
    // https://262.ecma-international.org/16.0/#prod-Punctuator
    fn js_lex_punctuator(&mut self) -> Result<Token<'a>, LexerError> {
        let ch = self.current();

        let token = match ch {
            '{' => {
                self.advance();

                Token::LeftBrace
            }
            '}' => {
                self.advance();

                Token::RightBrace
            }
            '(' => {
                self.advance();

                Token::LeftParen
            }
            ')' => {
                self.advance();

                Token::RightParen
            }
            '[' => {
                self.advance();

                Token::LeftBracket
            }
            ']' => {
                self.advance();

                Token::RightBracket
            }
            '.' => {
                self.advance();

                if self.advance_if_2('.', '.') {
                    Token::Spread
                } else {
                    Token::Dot
                }
            }
            ';' => {
                self.advance();

                Token::Semicolon
            }
            ',' => {
                self.advance();

                Token::Comma
            }
            '<' => {
                self.advance();

                if self.advance_if_2('<', '=') {
                    Token::LeftShiftAssign
                } else if self.advance_if('<') {
                    Token::LeftShift
                } else if self.advance_if('=') {
                    Token::LessThanEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                self.advance();

                if self.advance_if_3('>', '>', '=') {
                    Token::UnsignedRightShiftAssign
                } else if self.advance_if_2('>', '>') {
                    Token::UnsignedRightShift
                } else if self.advance_if_2('>', '=') {
                    Token::RightShiftAssign
                } else if self.advance_if('>') {
                    Token::RightShift
                } else if self.advance_if('=') {
                    Token::GreaterThanEqual
                } else {
                    Token::GreaterThan
                }
            }
            '=' => {
                self.advance();

                if self.advance_if_2('=', '=') {
                    Token::StrictEqual
                } else if self.advance_if('=') {
                    Token::Equal
                } else if self.advance_if('>') {
                    Token::Arrow
                } else {
                    Token::Assign
                }
            }
            '!' => {
                self.advance();

                if self.advance_if_2('=', '=') {
                    Token::StrictNotEqual
                } else if self.advance_if('=') {
                    Token::NotEqual
                } else {
                    Token::Not
                }
            }
            '+' => {
                self.advance();

                if self.advance_if('=') {
                    Token::PlusAssign
                } else if self.advance_if('+') {
                    Token::Increment
                } else {
                    Token::Plus
                }
            }
            '-' => {
                self.advance();

                if self.advance_if('=') {
                    Token::MinusAssign
                } else if self.advance_if('-') {
                    Token::Decrement
                } else {
                    Token::Minus
                }
            }
            '*' => {
                self.advance();

                if self.advance_if_2('*', '=') {
                    Token::ExponentAssign
                } else if self.advance_if('=') {
                    Token::MultiplyAssign
                } else if self.advance_if('*') {
                    Token::Exponent
                } else {
                    Token::Multiply
                }
            }
            '/' => {
                self.advance();

                if self.advance_if('=') {
                    Token::DivideAssign
                } else {
                    Token::Divide
                }
            }
            '%' => {
                self.advance();

                if self.advance_if('=') {
                    Token::ModuloAssign
                } else {
                    Token::Modulo
                }
            }
            '&' => {
                self.advance();

                if self.advance_if_2('&', '=') {
                    Token::LogicalAndAssign
                } else if self.advance_if('=') {
                    Token::BitAndAssign
                } else if self.advance_if('&') {
                    Token::LogicalAnd
                } else {
                    Token::BitAnd
                }
            }
            '|' => {
                self.advance();

                if self.advance_if_2('|', '=') {
                    Token::LogicalOrAssign
                } else if self.advance_if('=') {
                    Token::BitOrAssign
                } else if self.advance_if('|') {
                    Token::LogicalOr
                } else {
                    Token::BitOr
                }
            }
            '^' => {
                self.advance();

                if self.advance_if('=') {
                    Token::BitXorAssign
                } else {
                    Token::BitXor
                }
            }
            '~' => {
                self.advance();

                Token::Tilde
            }
            '?' => {
                self.advance();

                if self.advance_if_2('?', '=') {
                    Token::NullishCoalescingAssign
                } else if self.advance_if('?') {
                    Token::NullishCoalescing
                } else if self.advance_if('.') {
                    Token::OptionalChaining
                } else {
                    Token::Question
                }
            }
            ':' => {
                self.advance();

                Token::Colon
            }
            _ => Token::Illegal,
        };

        Ok(token)
    }

    // 12.9.3 Numeric Literals
    // https://262.ecma-international.org/16.0/#prod-NumericLiteral
    fn js_lex_number(&mut self) -> Result<Token<'a>, LexerError> {
        let start = self.current_byte_pos();

        let integer_end = self.js_read_number_fragment();

        let fractional_end = if self.advance_if('.') {
            Some(self.js_read_number_fragment())
        } else {
            None
        };

        let token = if let Some(fractional_end) = fractional_end {
            Token::Float64(self.source_str(start, fractional_end))
        } else {
            Token::Int64(self.source_str(start, integer_end))
        };

        Ok(token)
    }

    fn js_read_number_fragment(&mut self) -> usize {
        while !self.is_eof() && self.current().is_ascii_digit() {
            self.advance();
        }

        self.pos
    }

    // 12.9.4 String Literals
    // https://262.ecma-international.org/16.0/#prod-StringLiteral
    fn js_lex_string(&mut self) -> Result<Token<'a>, LexerError> {
        let start = self.current_byte_pos();

        let opening_quote_char = self.current();

        self.advance(); // Eat the opening quote.

        while !self.is_eof() {
            if self.current() == opening_quote_char {
                self.advance(); // Eat the closing quote.

                break;
            }

            self.advance();
        }

        Ok(Token::String(
            self.source_str(start, self.current_byte_pos()),
        ))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }

        self.js_skip_whitespace_and_line_terminators();

        if self.is_eof() {
            return Some(Token::Eof);
        }

        let token = match self.current() {
            '"' | '\'' => self.js_lex_string(),
            '0'..='9' => self.js_lex_number(),
            ch if is_char_punctuator_start(ch) => self.js_lex_punctuator(),
            ch if is_char_identifier_start(ch) => self.js_lex_identifier_name_or_keyword(),
            _ => self.error(LexerError::UnexpectedChar),
        };

        token.ok()
    }
}
