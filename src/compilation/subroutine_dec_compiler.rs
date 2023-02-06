use std::io::Write;

use anyhow::Result;

use crate::compilation::parameter_list_compiler::ParameterListCompiler;
use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// subroutineDec =(’constructor’ | ’function’ | ’method’) (’void’ | type) subroutineName ’(’ parameterList ’)’ subroutineBody
pub struct SubroutineDecCompiler {}

impl SubroutineDecCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’constructor’ | ’function’ | ’method’
        let subroutine_type = {
            tokenizer.advance()?;
            String::from(tokenizer.identifier())
        };

        // ’void’ | type
        tokenizer.advance()?;

        // subroutineName
        let subroutine_name = {
            tokenizer.advance()?;
            String::from(tokenizer.identifier())
        };

        // ’(’
        tokenizer.advance()?;
        // parameterList
        ParameterListCompiler::compile(tokenizer, symbol_tables)?;
        // ’)’
        tokenizer.advance()?;

        // subroutineBody
        SubroutineBodyCompiler::compile(
            tokenizer,
            symbol_tables,
            &subroutine_name,
            &subroutine_type,
            written,
        )?;

        Ok(())
    }
}
