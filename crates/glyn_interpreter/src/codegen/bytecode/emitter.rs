use crate::{
    codegen::bytecode::{generator::FinalProgram, instruction::Instruction},
    value::JSValue,
};

#[derive(Debug, Default)]
pub(crate) struct Emitter {
    constants: Vec<JSValue>,
    instructions: Vec<u8>,
}

impl Emitter {
    pub(crate) fn program(self) -> FinalProgram {
        FinalProgram {
            instructions: self.instructions,
            constants: self.constants,
        }
    }

    fn push(&mut self, instruction: u8) {
        self.instructions.push(instruction);
    }

    pub(crate) fn constant(&mut self, value: JSValue) {
        self.constants.push(value);

        self.push(Instruction::Const as u8);
        self.push(self.constants.len() as u8 - 1);
    }

    pub(crate) fn null(&mut self) {
        self.push(Instruction::Null as u8);
    }

    pub(crate) fn boolean(&mut self, value: bool) {
        self.push(if value {
            Instruction::True
        } else {
            Instruction::False
        } as u8);
    }

    pub(crate) fn unary_exp(&mut self, instruction: Instruction) {
        self.push(instruction as u8);
    }

    pub(crate) fn binary_exp(&mut self, instruction: Instruction) {
        self.push(instruction as u8);
    }
}
