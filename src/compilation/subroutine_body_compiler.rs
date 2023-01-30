use std::io::Write;

use anyhow::Result;

use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::var_dec_compiler::VarDecCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// subroutineBody = ’{’ varDec* statements ’}’
pub struct SubroutineBodyCompiler {}

impl SubroutineBodyCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’{’
        tokenizer.advance()?;

        // varDec*
        loop {
            if !KeyWord::exists(tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Var => VarDecCompiler::compile(tokenizer, symbol_tables)?,
                _ => break,
            }
        }

        // statements
        StatementsCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // ’}’
        tokenizer.advance()?;

        Ok(())
    }
}
