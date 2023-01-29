use std::io::Write;

use anyhow::Result;

use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// doStatement = ’do’ subroutineCall ’;’
pub struct DoStatementCompiler {}

impl DoStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // do
        tokenizer.advance()?;

        // subroutineCall
        SubroutineCallCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // ’;’
        tokenizer.advance()?;

        Ok(())
    }
}
