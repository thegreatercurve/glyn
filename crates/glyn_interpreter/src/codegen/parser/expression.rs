use crate::{
    codegen::{
        bytecode::instruction::Instruction,
        error::{CodeGenError, CodeGenResult},
        parser::Parser,
    },
    lexer::{BinOpPrecedence, Keyword, Token},
    value::string::JSString,
};

/// 13 ECMAScript Language: Expressions
/// https://262.ecma-international.org/16.0/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    /// 13.1 Identifiers
    /// https://262.ecma-international.org/16.0/#prod-IdentifierReference
    pub(crate) fn js_parse_identifier_reference(&mut self) -> CodeGenResult {
        let identifier_reference = self.current_token.to_string();

        if self.current_token.is_identifier_reference() {
            self.advance(); // Eat binding identifier token.

            // IdentifierReference : Identifier
            // 1. Return ? ResolveBinding(StringValue of Identifier).
            // IdentifierReference : yield
            // 1. Return ? ResolveBinding("yield").
            // IdentifierReference : await
            // 1. Return ? ResolveBinding("await").
            let identifier_reference_index = self
                .bytecode
                .add_identifier(JSString::from(identifier_reference));

            self.bytecode
                .emit_resolve_binding(identifier_reference_index);
        } else {
            return self.error(CodeGenError::UnexpectedToken);
        }

        Ok(())
    }

    /// https://262.ecma-international.org/16.0/#prod-BindingIdentifier
    pub(crate) fn js_parse_binding_identifier(&mut self) -> CodeGenResult<JSString> {
        let binding_identifier = self.current_token.to_string();

        if self.current_token.is_binding_identifier() {
            self.advance(); // Eat binding identifier token.
        } else {
            return self.error(CodeGenError::UnexpectedToken);
        }

        Ok(binding_identifier.into())
    }

    /// 13.15 Assignment Operators
    /// https://262.ecma-international.org/16.0/#prod-AssignmentExpression
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

    /// 13.16 Comma Operator ( , )
    /// https://262.ecma-international.org/16.0/#prod-Expression
    pub(crate) fn js_parse_expression(&mut self) -> CodeGenResult {
        self.js_parse_assignment_expression()
    }

    /// 13.2 Primary Expressions
    /// https://262.ecma-international.org/16.0/#prod-PrimaryExpression
    fn js_parse_primary_expression(&mut self) -> CodeGenResult {
        match &self.current_token {
            token if token.is_identifier_reference() => self.js_parse_identifier_reference(),
            _ => self.js_parse_literal(),
        }
    }

    /// 13.2.3 Literals
    /// https://262.ecma-international.org/16.0/#prod-Literal
    fn js_parse_literal(&mut self) -> CodeGenResult {
        use crate::value::JSValue;

        match self.current_token {
            Token::Keyword(Keyword::True) => {
                self.advance(); // Eat the literal token.

                self.bytecode.emit_instruction(Instruction::True);
            }
            Token::Keyword(Keyword::False) => {
                self.advance(); // Eat the literal token.

                self.bytecode.emit_instruction(Instruction::False);
            }
            Token::Keyword(Keyword::Null) => {
                self.advance(); // Eat the literal token.

                self.bytecode.emit_instruction(Instruction::Null);
            }
            Token::Int64(value) => {
                let f64_value = value
                    .parse::<f64>()
                    .map_err(|_| CodeGenError::InvalidInteger64Literal)?;

                self.advance(); // Eat the literal token.

                self.bytecode.emit_constant(JSValue::from(f64_value));
            }
            Token::String(value) => {
                let string_value = value.to_string();

                self.advance(); // Eat the literal token.

                self.bytecode.emit_constant(JSValue::from(string_value));
            }
            _ => self.error(CodeGenError::UnexpectedToken)?,
        };

        Ok(())
    }

    /// 13.3 Left-Hand-Side Expressions
    /// https://262.ecma-international.org/16.0/#prod-LeftHandSideExpression
    fn js_parse_left_hand_side_expression(&mut self) -> CodeGenResult {
        self.js_parse_primary_expression()
    }

    /// 13.4 Update Expressions
    /// https://262.ecma-international.org/16.0/#prod-UpdateExpression
    fn js_parse_update_expression(&mut self) -> CodeGenResult {
        self.js_parse_left_hand_side_expression()
    }

    /// 13.5 Unary Operators
    /// https://262.ecma-international.org/16.0/#prod-UnaryExpression
    fn js_parse_unary_expression(&mut self) -> CodeGenResult {
        match self.current_token {
            Token::Plus | Token::Minus => {
                let operation = self.current_token.clone();

                self.advance(); // Eat the unary operator token.

                self.js_parse_unary_expression()?;

                let instruction = match operation {
                    Token::Plus => Instruction::Plus,
                    Token::Minus => Instruction::Minus,
                    Token::Not => Instruction::Not,
                    _ => return Err(CodeGenError::UnexpectedToken),
                };

                self.bytecode.emit_instruction(instruction);

                Ok(())
            }
            _ => self.js_parse_update_expression(),
        }
    }

    /// 13.6 Exponentiation Operator
    /// https://262.ecma-international.org/16.0/#prod-ExponentiationExpression
    ///
    /// 13.7 Multiplicative Operators
    /// https://262.ecma-international.org/16.0/#prod-MultiplicativeExpression
    ///
    /// 13.8 Additive Operators
    /// https://262.ecma-international.org/16.0/#prod-AdditiveExpression
    ///
    /// 13.9 Bitwise Shift Operators
    /// https://262.ecma-international.org/16.0/#prod-ShiftExpression
    ///
    /// 13.10 Relational Operators
    /// https://262.ecma-international.org/16.0/#prod-RelationalExpression
    ///
    /// 13.11 Equality Operators
    /// https://262.ecma-international.org/16.0/#prod-EqualityExpression
    ///
    /// 13.12 Binary Bitwise Operators
    /// https://262.ecma-international.org/16.0/#prod-BitwiseANDExpression
    /// https://262.ecma-international.org/16.0/#prod-BitwiseXORExpression
    /// https://262.ecma-international.org/16.0/#prod-BitwiseORExpression
    ///
    /// 13.13 Binary Logical Operators
    /// https://262.ecma-international.org/16.0/#prod-LogicalANDExpression
    /// https://262.ecma-international.org/16.0/#prod-LogicalORExpression
    ///
    /// 13.14 Conditional Operator ( ? : )
    /// https://262.ecma-international.org/16.0/#prod-ConditionalExpression
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

            let instruction = match operator {
                Token::Plus => Instruction::BinAdd,
                Token::Minus => Instruction::BinSubtract,
                Token::Multiply => Instruction::BinMultiply,
                Token::Divide => Instruction::BinDivide,
                Token::Exponent => Instruction::BinExponent,
                Token::Modulo => Instruction::BinModulo,
                Token::Equal => Instruction::Equal,
                Token::NotEqual => Instruction::NotEqual,
                Token::StrictEqual => Instruction::StrictEqual,
                Token::StrictNotEqual => Instruction::StrictNotEqual,
                Token::LessThan => Instruction::LessThan,
                Token::LessThanEqual => Instruction::LessThanOrEqual,
                Token::GreaterThan => Instruction::GreaterThan,
                Token::GreaterThanEqual => Instruction::GreaterThanOrEqual,
                Token::BitAnd => Instruction::BitAnd,
                Token::BitOr => Instruction::BitOr,
                Token::BitXor => Instruction::BitXor,
                Token::LeftShift => Instruction::BitShiftLeft,
                Token::RightShift => Instruction::BitShiftRight,
                Token::UnsignedRightShift => Instruction::BitShiftRight,
                Token::LogicalAnd => Instruction::LogicalAnd,
                Token::LogicalOr => Instruction::LogicalOr,
                _ => return Err(CodeGenError::UnexpectedToken),
            };

            self.bytecode.emit_instruction(instruction);
        }

        Ok(())
    }
}
