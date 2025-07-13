use std::collections::HashSet;

use crate::{
    codegen::bytecode::{generator::FinalProgram, instruction::Instruction},
    value::{string::JSString, JSValue},
};

#[derive(Debug, Default)]
pub(crate) struct Emitter {
    instructions: Vec<u8>,
    constants: Vec<JSValue>,
    identifiers: Vec<String>,
}

impl Emitter {
    pub(crate) fn program(self) -> FinalProgram {
        FinalProgram {
            instructions: self.instructions,
            constants: self.constants,
            identifiers: self.identifiers,
        }
    }

    fn push(&mut self, instruction: u8) {
        self.instructions.push(instruction);
    }

    pub(crate) fn identifier(&mut self, identifier: JSString) -> u8 {
        self.identifiers.push(identifier.0);

        (self.identifiers.len() - 1) as u8
    }

    pub(crate) fn constant(&mut self, value: JSValue) {
        self.constants.push(value);

        self.push(Instruction::Const as u8);
        self.push(self.constants.len() as u8 - 1);
    }

    pub(crate) fn null(&mut self) {
        self.push(Instruction::Null as u8);
    }

    pub(crate) fn undefined(&mut self) {
        self.push(Instruction::Undefined as u8);
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

    pub(crate) fn resolve_binding(&mut self, identifier_index: u8) {
        self.push(Instruction::ResolveBinding as u8);
        self.push(identifier_index);
    }

    pub(crate) fn initialize_referenced_binding(&mut self) {
        self.push(Instruction::InitializeReferencedBinding as u8);
    }
}
