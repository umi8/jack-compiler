use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Let;

/// letStatement = ’let’ varName (’[’ expression ’]’)? ’=’ expression ’;’
pub struct LetStatementCompiler {}

impl LetStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <letStatement>
        writer.write_start_tag("letStatement", written)?;
        // let
        writer.write_key_word(tokenizer, vec![Let], written)?;
        // varName
        writer.write_identifier(tokenizer, symbol_tables, written)?;
        // (’[’ expression ’]’)?
        if tokenizer.peek()?.value() == "[" {
            // ’[’
            writer.write_symbol(tokenizer, written)?;
            // expression
            ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
            // ’]’
            writer.write_symbol(tokenizer, written)?;
        }
        // ’=’
        writer.write_symbol(tokenizer, written)?;
        // expression
        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </letStatement>
        writer.write_end_tag("letStatement", written)?;
        Ok(())
    }
}
