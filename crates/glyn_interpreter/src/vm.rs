use crate::{
    abstract_ops::runtime_operations::apply_string_or_numeric_binary_operator_string_concat,
    codegen::bytecode::{generator::FinalProgram, instruction::Instruction},
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
    RuntimeError(String),
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
            Instruction::Const => self.execute_const(),
            Instruction::Add => self.execute_add()?,
            Instruction::Halt => self.running = false,
            _ => {}
        };

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

    fn execute_const(&mut self) {
        let index = self.read_byte();

        let value = self.get_constant(index);

        self.push(value);
    }

    fn execute_add(&mut self) -> VMResult {
        let b = self.pop()?;
        let a = self.pop()?;

        let result = apply_string_or_numeric_binary_operator_string_concat(self.agent, a, b)
            .map_err(|e| VMError::RuntimeError(format!("Unable to apply addition to: {e:?}")))?;

        self.push(result);

        Ok(())
    }
}
