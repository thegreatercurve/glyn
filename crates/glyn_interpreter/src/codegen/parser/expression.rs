use crate::{
    codegen::{
        bytecode::generator::LiteralType,
        error::{CodeGenError, CodeGenResult},
        parser::Parser,
    },
    lexer::{BinOpPrecedence, Keyword, Token},
    value::string::JSString,
};

enum Literal {
    True,
    False,
    Int64(f64),
    String,
    Null,
}

// 13 ECMAScript Language: Expressions
// https://262.ecma-international.org/16.0/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 13.1 Identifiers
    // https://262.ecma-international.org/16.0/#prod-IdentifierReference
    pub(crate) fn js_parse_identifier_reference(&mut self) -> CodeGenResult<JSString> {
        let binding_identifier = self.current_token.to_string();

        if self.current_token.is_identifier_reference() {
            self.advance(); // Eat binding identifier token.
        } else {
            return self.error(CodeGenError::UnexpectedToken);
        }

        Ok(binding_identifier.into())
    }

    // https://262.ecma-international.org/16.0/#prod-BindingIdentifier
    pub(crate) fn js_parse_binding_identifier(&mut self) -> CodeGenResult<JSString> {
        let binding_identifier = self.current_token.to_string();

        if self.current_token.is_binding_identifier() {
            self.advance(); // Eat binding identifier token.
        } else {
            return self.error(CodeGenError::UnexpectedToken);
        }

        Ok(binding_identifier.into())
    }

    // 13.15 Assignment Operators
    // https://262.ecma-international.org/16.0/#prod-AssignmentExpression
    pub(crate) fn js_parse_assignment_expression(&mut self) -> CodeGenResult {
        self.js_parse_conditional_expression()?;

        let operator = if self.current_token.is_assignment_operator() {
            self.advance(); // Eat the assignment operator token.

            self.current_token.clone()
        } else {
            return Ok(());
        };

        self.js_parse_assignment_expression()?;

        Ok(())
    }

    // 13.16 Comma Operator ( , )
    // https://262.ecma-international.org/16.0/#prod-Expression
    pub(crate) fn js_parse_expression(&mut self) -> CodeGenResult {
        self.js_parse_assignment_expression()
    }

    // 13.2 Primary Expressions
    // https://262.ecma-international.org/16.0/#prod-PrimaryExpression
    fn js_parse_primary_expression(&mut self) -> CodeGenResult {
        match &self.current_token {
            // token if token.is_identifier_reference() => {
            //     let _ident = self.js_parse_identifier_reference()?;

            //     self.bytecode
            //         .compile_get_let_variable()
            //         .map_err(CodeGenError::from)?;

            //     Ok(())
            // }
            _ => self.js_parse_literal(),
        }
    }

    // 13.2.3 Literals
    // https://262.ecma-international.org/16.0/#prod-Literal
    fn js_parse_literal(&mut self) -> CodeGenResult {
        let literal_type = match self.current_token {
            Token::Keyword(Keyword::True) => LiteralType::Boolean(true),
            Token::Keyword(Keyword::False) => LiteralType::Boolean(false),
            Token::Keyword(Keyword::Null) => LiteralType::Null,
            Token::Int64(value) => {
                let f64_value = value
                    .parse::<f64>()
                    .map_err(|_| CodeGenError::InvalidInteger64Literal)?;

                LiteralType::Int64(f64_value)
            }
            Token::String(value) => LiteralType::String(value.to_string()),
            _ => self.error(CodeGenError::UnexpectedToken)?,
        };

        self.advance(); // Eat the literal token.

        self.bytecode.generate_literal(&literal_type)?;

        Ok(())
    }

    // 13.3 Left-Hand-Side Expressions
    // https://262.ecma-international.org/16.0/#prod-LeftHandSideExpression
    fn js_parse_left_hand_side_expression(&mut self) -> CodeGenResult {
        self.js_parse_primary_expression()
    }

    // 13.4 Update Expressions
    // https://262.ecma-international.org/16.0/#prod-UpdateExpression
    fn js_parse_update_expression(&mut self) -> CodeGenResult {
        self.js_parse_left_hand_side_expression()
    }

    // 13.5 Unary Operators
    // https://262.ecma-international.org/16.0/#prod-UnaryExpression
    fn js_parse_unary_expression(&mut self) -> CodeGenResult {
        match self.current_token {
            Token::Plus | Token::Minus => {
                let operation = self.current_token.clone();

                self.advance(); // Eat the unary operator token.

                self.js_parse_unary_expression()?;

                self.bytecode.generate_unary_exp(&operation)
            }
            _ => self.js_parse_update_expression(),
        }
    }

    // 13.6 Exponentiation Operator
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

    // 13.14 Conditional Operator ( ? : )
    // https://262.ecma-international.org/16.0/#prod-ConditionalExpression
    fn js_parse_conditional_expression(&mut self) -> CodeGenResult {
        self.js_parse_binary_expression(BinOpPrecedence::Lowest)
    }

    fn js_parse_binary_expression(&mut self, precedence: BinOpPrecedence) -> CodeGenResult {
        self.js_parse_unary_expression()?;

        if !self.current_token.is_binary_operator() {
            return Ok(());
        }

        self.js_parse_binary_expression_rest(precedence)
    }

    fn js_parse_binary_expression_rest(&mut self, precedence: BinOpPrecedence) -> CodeGenResult {
        while !self.is_eof() {
            let operator = self.current_token.clone();

            let new_precedence = BinOpPrecedence::from(operator.clone());

            let stop = if new_precedence.is_right_associative() {
                new_precedence < precedence
            } else {
                new_precedence <= precedence
            };

            if stop {
                break;
            }

            self.advance(); // Eat the binary operator token.

            self.js_parse_binary_expression(new_precedence)?;

            self.bytecode.generate_binary_exp(&operator)?;
        }

        Ok(())
    }
}
