use crate::{
    codegen::bytecode::{generator::FinalProgram, instruction::Instruction},
    runtime::agent::JSAgent,
    value::JSValue,
};

pub(crate) struct VM<'a> {
    agent: &'a mut JSAgent,
    stack: Vec<JSValue>,
    program: FinalProgram,
    locals: Vec<JSValue>,
    ip: usize,
    running: bool,
}

impl<'a> VM<'a> {
    pub(crate) fn new(agent: &'a mut JSAgent, program: FinalProgram) -> Self {
        Self {
            agent,
            stack: Vec::with_capacity(32),
            locals: Vec::with_capacity(32),
            program,
            ip: 0,
            running: false,
        }
    }

    pub(crate) fn evaluate_script(&mut self) -> Option<JSValue> {
        self.running = true;

        while self.running && self.ip < self.program.instructions.len() {
            self.instruction();
        }

        self.pop()
    }

    fn instruction(&mut self) {
        let instruction = self.program.instructions[self.ip].into();

        self.ip += 1;

        match instruction {
            Instruction::Halt => self.running = false,
            _ => {}
        };
    }

    fn read_byte(&mut self) -> u8 {
        let value = self.program.instructions[self.ip];

        self.ip += 1;

        value
    }

    fn push(&mut self, value: JSValue) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<JSValue> {
        self.stack.pop()
    }

    fn execute_const(&mut self) {
        let index = self.read_byte() as usize;

        let value = &self.program.constants[index];
    }
}
