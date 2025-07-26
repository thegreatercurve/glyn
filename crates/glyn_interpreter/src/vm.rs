use crate::{
    abstract_ops::{
        execution_contexts::resolve_binding,
        reference_operations::initialize_referenced_binding,
        runtime_operations::{
            apply_numeric_binary_operator, apply_string_or_numeric_binary_operator,
        },
        testing_comparison::{is_less_than, is_loosely_equal, is_strictly_equal},
    },
    codegen::bytecode::{
        generator::{ExecutableProgram, Identifier},
        instruction::Instruction,
    },
    lexer::Token,
    runtime::{agent::JSAgent, reference::Reference},
    value::{number::JSNumber, string::JSString, JSValue},
};

pub(crate) struct VM<'a> {
    agent: &'a mut JSAgent,
    stack: Vec<JSValue>,
    references: Vec<Reference>,
    program: &'a ExecutableProgram,
    ip: usize,
    running: bool,
}

pub(crate) enum VMError {
    BinOperationError,
    InitializeReferencedBindingError,
    LessThanComparisonError,
    LooselyEqualComparisonError,
    ReferenceError,
    StackUnderflow,
    UnaryOperationError,
    UnexpectedInstruction,
}

type VMResult<T = ()> = Result<T, VMError>;

impl<'a> VM<'a> {
    pub(crate) fn new(agent: &'a mut JSAgent, program: &'a ExecutableProgram) -> Self {
        Self {
            agent,
            stack: Vec::with_capacity(32),
            references: Vec::with_capacity(32),
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
            Instruction::Undefined => self.exec_undefined(),
            Instruction::Add => self.exec_bin_add(),
            Instruction::Subtract => self.exec_numeric_bin_op(Token::Minus),
            Instruction::Multiply => self.exec_numeric_bin_op(Token::Multiply),
            Instruction::Divide => self.exec_numeric_bin_op(Token::Divide),
            Instruction::Modulo => self.exec_numeric_bin_op(Token::Modulo),
            Instruction::Exponent => self.exec_numeric_bin_op(Token::Exponent),
            Instruction::StrictEqual => self.exec_strictly_equal(true),
            Instruction::StrictNotEqual => self.exec_strictly_equal(false),
            Instruction::Equal => self.exec_loosely_equal(true),
            Instruction::NotEqual => self.exec_loosely_equal(false),
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
            Instruction::ResolveBinding => self.exec_resolve_binding(),
            Instruction::InitializeReferencedBinding => self.exec_initialize_referenced_binding(),
            Instruction::Halt => {
                self.running = false;

                Ok(())
            }
            _ => return Err(VMError::UnexpectedInstruction),
        }?;

        #[cfg(feature = "debug")]
        {
            println!("{}", instruction);
            println!(
                "Constants: {:?} | Identifiers: {:?} | Stack: {:?}",
                self.program.constants, self.program.identifiers, self.stack
            );
            println!();
        }

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

    fn get_identifier(&mut self, index: u8) -> &Identifier {
        &self.program.identifiers[index as usize]
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

    fn push_reference(&mut self, binding: Reference) {
        self.references.push(binding);
    }

    fn pop_reference(&mut self) -> VMResult<Reference> {
        self.references.pop().ok_or(VMError::ReferenceError)
    }

    fn exec_const(&mut self) -> VMResult {
        let index = self.read_byte();

        let value = self.get_constant(index);

        self.push(value);

        Ok(())
    }

    fn exec_bin_add(&mut self) -> VMResult {
        let (a, b) = self.pop_two()?;

        let result = apply_string_or_numeric_binary_operator(a, b)
            .map_err(|_| VMError::BinOperationError)?;

        self.push(result);

        Ok(())
    }

    fn exec_numeric_bin_op(&mut self, operator: Token) -> VMResult {
        let (a, b) = self.pop_two()?;

        let result = apply_numeric_binary_operator(a, operator, b)
            .map_err(|_| VMError::BinOperationError)?;

        self.push(result);

        Ok(())
    }

    /// 13.5.5.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-unary-minus-operator-runtime-semantics-evaluation
    /// UnaryExpression : - UnaryExpression
    fn exec_unary_minus(&mut self) -> VMResult {
        let value = self.pop()?;

        let number = JSNumber::try_from(value)
            .map_err(|_| VMError::UnaryOperationError)?
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
        let result = is_less_than(a, b, true)
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
        let result = is_less_than(b, a, false)
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
        let result = !is_less_than(b, a, false)
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
        let result = !is_less_than(a, b, true)
            .map_err(|_| VMError::LessThanComparisonError)?
            // 6. If r is either true or undefined, return false. Otherwise, return true.
            .unwrap_or(true);

        self.push(JSValue::from(result));

        Ok(())
    }

    /// 13.11.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-equality-operators-runtime-semantics-evaluation
    /// EqualityExpression : EqualityExpression == RelationalExpression
    /// EqualityExpression : EqualityExpression != RelationalExpression
    fn exec_loosely_equal(&mut self, check_equal: bool) -> VMResult {
        let (a, b) = self.pop_two()?;

        let result = is_loosely_equal(a, b).map_err(|_| VMError::LooselyEqualComparisonError)?;

        self.push(JSValue::from(if check_equal { result } else { !result }));

        Ok(())
    }

    /// 13.11.1 Runtime Semantics: Evaluation
    /// https://262.ecma-international.org/16.0/#sec-equality-operators-runtime-semantics-evaluation
    /// EqualityExpression : EqualityExpression === RelationalExpression
    /// EqualityExpression : EqualityExpression !== RelationalExpression
    fn exec_strictly_equal(&mut self, check_equal: bool) -> VMResult {
        let (a, b) = self.pop_two()?;

        // 5. Return IsStrictlyEqual(rval, lval).
        let result = is_strictly_equal(&a, &b);

        self.push(JSValue::from(if check_equal { result } else { !result }));

        Ok(())
    }

    fn exec_resolve_binding(&mut self) -> VMResult {
        let index = self.read_byte();

        let value = String::from(self.get_identifier(index));

        let binding = resolve_binding(
            self.agent,
            &JSString::from(value),
            self.agent
                .running_execution_context()
                .lexical_environment
                .clone(),
        )
        .map_err(|_| VMError::ReferenceError)?;

        self.push_reference(binding);

        Ok(())
    }

    fn exec_initialize_referenced_binding(&mut self) -> VMResult {
        let reference = self.pop_reference()?;
        let value = self.pop()?;

        initialize_referenced_binding(reference, value)
            .map_err(|_| VMError::InitializeReferencedBindingError)?;

        Ok(())
    }

    fn exec_undefined(&mut self) -> VMResult {
        self.push(JSValue::Undefined);

        Ok(())
    }
}
