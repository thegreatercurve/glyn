use crate::{
    abstract_ops::{
        runtime_operations::{
            apply_numeric_binary_operator, apply_string_or_numeric_binary_operator,
        },
        testing_comparison::{is_less_than, is_strictly_equal},
    },
    codegen::bytecode::{generator::FinalProgram, instruction::Instruction},
    lexer::Token,
    runtime::agent::JSAgent,
    value::JSValue,
};

pub(crate) struct VM<'a> {
    agent: &'a mut JSAgent,
    stack: Vec<JSValue>,
    program: &'a FinalProgram,
    locals: Vec<JSValue>,
    ip: usize,
    running: bool,
}

pub(crate) enum VMError {
    StackUnderflow,
    BinOperationError,
    UnaryOperationError,
    LessThanComparisonError,
    UnexpectedInstruction,
}

type VMResult<T = ()> = Result<T, VMError>;

impl<'a> VM<'a> {
    pub(crate) fn new(agent: &'a mut JSAgent, program: &'a FinalProgram) -> Self {
        Self {
            agent,
            stack: Vec::with_capacity(32),
            locals: Vec::with_capacity(32),
            program,
            ip: 0,
            running: false,
        }
    }

    pub(crate) fn evaluate_script(&mut self) -> VMResult<JSValue> {
        self.running = true;

        while self.running && self.ip < self.program.instructions.len() {
            self.instruction()?;
        }

        self.pop()
    }

    fn instruction(&mut self) -> VMResult {
        let instruction = self.program.instructions[self.ip].into();

        self.ip += 1;

        match instruction {
            Instruction::Const => self.exec_const(),
            Instruction::Add => self.exec_bin_add(),
            Instruction::Subtract => self.exec_numeric_bin_op(Token::Minus),
            Instruction::Multiply => self.exec_numeric_bin_op(Token::Multiply),
            Instruction::Divide => self.exec_numeric_bin_op(Token::Divide),
            Instruction::Modulo => self.exec_numeric_bin_op(Token::Modulo),
            Instruction::Exponent => self.exec_numeric_bin_op(Token::Exponent),
            Instruction::StrictEqual => self.exec_strictly_equal(),
            Instruction::StrictNotEqual => self.exec_strictly_not_equal(),
            Instruction::LessThan => self.exec_less_than(),
            Instruction::LessThanOrEqual => self.exec_less_than_or_equal(),
            Instruction::GreaterThan => self.exec_greater_than(),
            Instruction::GreaterThanOrEqual => self.exec_greater_than_or_equal(),
            Instruction::Plus => Ok(()), // No-op,
            Instruction::Minus => self.exec_unary_minus(),
            Instruction::BitAnd => self.exec_numeric_bin_op(Token::BitAnd),
            Instruction::BitOr => self.exec_numeric_bin_op(Token::BitOr),
            Instruction::BitXor => self.exec_numeric_bin_op(Token::BitXor),
            Instruction::BitShiftLeft => self.exec_numeric_bin_op(Token::LeftShift),
            Instruction::BitShiftRight => self.exec_numeric_bin_op(Token::RightShift),
            Instruction::BitShiftRightUnsigned => {
                self.exec_numeric_bin_op(Token::UnsignedRightShift)
            }
            Instruction::Halt => {
                self.running = false;

                Ok(())
            }
            _ => return Err(VMError::UnexpectedInstruction),
        }?;

        Ok(())
    }

    fn read_byte(&mut self) -> u8 {
        let value = self.program.instructions[self.ip];

        self.ip += 1;

        value
    }

    fn get_constant(&mut self, index: u8) -> JSValue {
        self.program.constants[index as usize].clone()
    }

    fn push(&mut self, value: JSValue) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> VMResult<JSValue> {
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }

    fn pop_two(&mut self) -> VMResult<(JSValue, JSValue)> {
        let b = self.pop()?;
        let a = self.pop()?;

        Ok((a, b))
    }

    fn exec_const(&mut self) -> VMResult {
        let index = self.read_byte();

        let value = self.get_constant(index);

        self.push(value);

        Ok(())
    }

    fn exec_bin_add(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        let result = apply_string_or_numeric_binary_operator(self.agent, a, b)
            .map_err(|_| VMError::BinOperationError)?;

        self.push(result);

        Ok(())
    }

    fn exec_numeric_bin_op(&mut self, operator: Token) -> VMResult {
        let (a, b) = self.pop_two()?;

        let result = apply_numeric_binary_operator(self.agent, a, operator, b)
            .map_err(|_| VMError::BinOperationError)?;

        self.push(result);

        Ok(())
    }

    /// 13.5.5.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-unary-minus-operator-runtime-semantics-evaluation
    /// UnaryExpression : - UnaryExpression
    fn exec_unary_minus(&mut self) -> VMResult {
        let value = self.pop()?;

        let number = value
            .as_number()
            .ok_or(VMError::UnaryOperationError)?
            .unary_minus();

        self.push(JSValue::Number(number));

        Ok(())
    }

    /// 13.10.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-relational-operators-runtime-semantics-evaluation
    /// RelationalExpression : RelationalExpression < ShiftExpression
    fn exec_less_than(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Let r be ? IsLessThan(lval, rval, true).
        let result = is_less_than(self.agent, a, b, true)
            .map_err(|_| VMError::LessThanComparisonError)?
            // 6. If r is undefined, return false. Otherwise, return r.
            .unwrap_or(false);

        self.push(JSValue::from(result));

        Ok(())
    }

    /// 13.10.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-relational-operators-runtime-semantics-evaluation
    /// RelationalExpression : RelationalExpression > ShiftExpression
    fn exec_greater_than(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Let r be ? IsLessThan(rval, lval, false).
        let result = is_less_than(self.agent, b, a, false)
            .map_err(|_| VMError::LessThanComparisonError)?
            // 6. If r is undefined, return false. Otherwise, return r.
            .unwrap_or(false);

        self.push(JSValue::from(result));

        Ok(())
    }

    /// 13.10.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-relational-operators-runtime-semantics-evaluation
    /// RelationalExpression : RelationalExpression <= ShiftExpression
    fn exec_less_than_or_equal(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Let r be ? IsLessThan(rval, lval, false).
        let result = !is_less_than(self.agent, b, a, false)
            .map_err(|_| VMError::LessThanComparisonError)?
            // 6. If r is either true or undefined, return false. Otherwise, return true.
            .unwrap_or(true);

        self.push(JSValue::from(result));

        Ok(())
    }

    /// 13.10.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-relational-operators-runtime-semantics-evaluation
    /// RelationalExpression : RelationalExpression >= ShiftExpression
    fn exec_greater_than_or_equal(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Let r be ? IsLessThan(lval, rval, true).
        let result = !is_less_than(self.agent, a, b, true)
            .map_err(|_| VMError::LessThanComparisonError)?
            // 6. If r is either true or undefined, return false. Otherwise, return true.
            .unwrap_or(true);

        self.push(JSValue::from(result));

        Ok(())
    }

    /// 13.11.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-equality-operators-runtime-semantics-evaluation
    /// EqualityExpression : EqualityExpression === RelationalExpression
    fn exec_strictly_equal(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Return IsStrictlyEqual(rval, lval).
        let result = is_strictly_equal(&a, &b);

        self.push(JSValue::from(result));

        Ok(())
    }

    /// 13.11.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-equality-operators-runtime-semantics-evaluation
    /// EqualityExpression : EqualityExpression !== RelationalExpression
    fn exec_strictly_not_equal(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Return IsStrictlyEqual(rval, lval).
        let result = is_strictly_equal(&a, &b);

        // 6. If r is true, return false. Otherwise, return true.
        self.push(JSValue::from(!result));

        Ok(())
    }
}
