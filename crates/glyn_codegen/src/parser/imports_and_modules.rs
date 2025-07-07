use crate::parser::{CodeGenResult, Parser};

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
pub enum ProgramSource {
    Script,
    Module,
}

impl<'a> Parser<'a> {
    // 16.1 Scripts
    // https://tc39.es/ecma262/#prod-Script
    pub(crate) fn js_parse_script(&mut self) -> CodeGenResult {
        // TODO Parse prologue if present.

        self.js_parse_statement_list()?;

        Ok(())
    }
}
