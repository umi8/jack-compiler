use std::io::Write;

use anyhow::Result;

use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// doStatement = ’do’ subroutineCall ’;’
pub struct DoStatementCompiler {}

impl DoStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // do
        tokenizer.advance()?;

        // subroutineCall
        SubroutineCallCompiler::compile(tokenizer, symbol_tables, written)?;

        // After the called function returns,
        // the caller's memory segments-argument, local, static, this, that, and pointer-are
        // the same as before the function call.
        // However, the temp segment is undefined, so it must be defined.
        VmWriter::write_pop(&Segment::Temp, 0, written)?;

        // ’;’
        tokenizer.advance()?;

        Ok(())
    }
}
