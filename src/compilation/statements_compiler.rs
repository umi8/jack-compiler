use std::io::Write;

use anyhow::Result;

use crate::compilation::statement_compiler::StatementCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// statements = statement*
pub struct StatementsCompiler {}

impl StatementsCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        while KeyWord::exists(tokenizer.peek()?.value()) {
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Let | KeyWord::If | KeyWord::While | KeyWord::Do | KeyWord::Return => {
                    StatementCompiler::compile(tokenizer, symbol_tables, written)?;
                }
                _ => break,
            }
        }
        Ok(())
    }
}
