use crate::codegen::parser::{CodeGenResult, Parser};

/// 16 ECMAScript Language: Scripts and Modules
/// https://262.ecma-international.org/16.0/#sec-ecmascript-language-scripts-and-modules
pub(crate) enum ProgramSource {
    Script,
    Module,
}

impl<'a> Parser<'a> {
    /// 16.1 Scripts
    /// https://262.ecma-international.org/16.0/#prod-Script
    pub(crate) fn js_parse_script(&mut self) -> CodeGenResult {
        // TODO Parse prologue if present.

        self.js_parse_statement_list()?;

        Ok(())
    }
}
