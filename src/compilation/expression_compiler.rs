use std::io::Write;

use anyhow::Result;

use crate::compilation::term_compiler::TermCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// expression = term (op term)*
pub struct ExpressionCompiler {}

impl ExpressionCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <expression>
        writer.write_start_tag("expression", written)?;
        // term
        TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // (op term)*
        loop {
            if tokenizer.peek()?.is_op() {
                // op
                writer.write_symbol(tokenizer, written)?;
                // term
                TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;
            } else {
                break;
            }
        }
        // </expression>
        writer.write_end_tag("expression", written)?;
        Ok(())
    }
}
