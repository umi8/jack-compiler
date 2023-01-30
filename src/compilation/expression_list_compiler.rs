use std::io::Write;

use crate::compilation::expression_compiler::ExpressionCompiler;
use anyhow::Result;

use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// expressionList = (expression (’,’ expression)* )?
pub struct ExpressionListCompiler {}

impl ExpressionListCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // (expression)?
        if tokenizer.is_term()? {
            // expression
            ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;

            // (’,’ expression)*
            while tokenizer.peek()?.value() == "," {
                // ’,’
                tokenizer.advance()?;

                // expression
                ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
            }
        }

        Ok(())
    }
}
