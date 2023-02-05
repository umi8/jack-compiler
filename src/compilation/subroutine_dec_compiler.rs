use std::io::Write;

use anyhow::Result;

use crate::compilation::parameter_list_compiler::ParameterListCompiler;
use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// subroutineDec =(’constructor’ | ’function’ | ’method’) (’void’ | type) subroutineName ’(’ parameterList ’)’ subroutineBody
pub struct SubroutineDecCompiler {}

impl SubroutineDecCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’constructor’ | ’function’ | ’method’
        tokenizer.advance()?;
        let subroutine_type = String::from(tokenizer.identifier());

        let subroutine_name = {
            // ’void’ | type
            tokenizer.advance()?;
            // subroutineName
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
            writer,
            symbol_tables,
            &subroutine_name,
            &subroutine_type,
            written,
        )?;

        Ok(())
    }
}
