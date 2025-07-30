use crate::{
    codegen::bytecode::instruction::Instruction,
    value::{string::JSString, JSValue},
};

#[derive(Clone, Debug, Default)]
pub(crate) struct ExecutableProgram {
    pub(crate) instructions: Vec<u8>,
    pub(crate) constants: Vec<JSValue>,
    pub(crate) identifiers: Vec<JSString>,
}

#[derive(Debug, Default)]
pub(crate) struct BytecodeGenerator {
    instructions: Vec<u8>,
    constants: Vec<JSValue>,
    identifiers: Vec<JSString>,
    scope_depth: u8,
}

impl BytecodeGenerator {
    pub(crate) fn program(self) -> ExecutableProgram {
        ExecutableProgram {
            instructions: self.instructions,
            constants: self.constants,
            identifiers: self.identifiers,
        }
    }

    fn push(&mut self, instruction: u8) {
        self.instructions.push(instruction);
    }

    pub(crate) fn add_identifier(&mut self, identifier: JSString) -> u8 {
        self.identifiers.push(identifier);

        (self.identifiers.len() - 1) as u8
    }

    pub(crate) fn add_constant(&mut self, constant: JSValue) {
        self.constants.push(constant);
    }

    pub(crate) fn emit_instruction(&mut self, instruction: Instruction) {
        self.push(instruction as u8);
    }

    pub(crate) fn emit_constant(&mut self, value: JSValue) {
        self.add_constant(value);

        self.push(Instruction::Const as u8);

        self.push(self.constants.len() as u8 - 1);
    }

    pub(crate) fn emit_resolve_binding(&mut self, identifier_index: u8) {
        self.push(Instruction::ResolveBinding as u8);

        self.push(identifier_index);
    }

    pub(crate) fn emit_create_mutable_binding(&mut self, binding_index: u8) {
        self.push(Instruction::CreateMutableBinding as u8);

        self.push(binding_index);

        self.push(self.scope_depth);
    }

    pub(crate) fn emit_initialize_referenced_binding(&mut self) {
        self.push(Instruction::InitializeReferencedBinding as u8);
    }

    pub(crate) fn emit_call(&mut self, args_length: u8) {
        self.push(Instruction::Call as u8);

        self.push(args_length);
    }
}
