use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::While;

/// whileStatement = ’while’ ’(’ expression ’)’ ’{’ statements ’}’
pub struct WhileStatementCompiler {}

impl WhileStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <whileStatement>
        writer.write_start_tag("whileStatement", written)?;
        // while
        writer.write_key_word(tokenizer, vec![While], written)?;
        // ’(’
        writer.write_symbol(tokenizer, written)?;
        // expression
        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // ’)’
        writer.write_symbol(tokenizer, written)?;
        // ’{’
        writer.write_symbol(tokenizer, written)?;
        // statements
        StatementsCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // ’}’
        writer.write_symbol(tokenizer, written)?;
        // </whileStatement>
        writer.write_end_tag("whileStatement", written)?;
        Ok(())
    }
}
