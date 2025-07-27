use crate::{
    codegen::{bytecode::instruction::Instruction, error::CodeGenResult},
    value::{string::JSString, JSValue},
};

#[derive(Clone, Debug)]
pub(crate) enum Identifier {
    Var(String),
    Let(String),
    Const(String),
}

impl Identifier {
    pub(crate) fn is_lexical_declaration(&self) -> bool {
        matches!(self, Identifier::Let(_) | Identifier::Const(_))
    }

    pub(crate) fn is_constant_declaration(&self) -> bool {
        matches!(self, Identifier::Const(_))
    }

    pub(crate) fn is_variable_declaration(&self) -> bool {
        matches!(self, Identifier::Var(_))
    }
}

impl From<&Identifier> for String {
    fn from(identifier: &Identifier) -> String {
        match identifier {
            Identifier::Var(name) => name.clone(),
            Identifier::Let(name) => name.clone(),
            Identifier::Const(name) => name.clone(),
        }
    }
}

impl From<&Identifier> for JSString {
    fn from(identifier: &Identifier) -> JSString {
        JSString(String::from(identifier))
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ExecutableProgram {
    pub(crate) instructions: Vec<u8>,
    pub(crate) constants: Vec<JSValue>,
    pub(crate) identifiers: Vec<Identifier>,
}

#[derive(Debug, Default)]
pub(crate) struct BytecodeGenerator {
    instructions: Vec<u8>,
    constants: Vec<JSValue>,
    identifiers: Vec<Identifier>,
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

    pub(crate) fn add_identifier(&mut self, identifier: Identifier) -> u8 {
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

    pub(crate) fn emit_initialize_referenced_binding(&mut self) {
        self.push(Instruction::InitializeReferencedBinding as u8);
    }
}
