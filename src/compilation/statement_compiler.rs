use std::io::Write;

use anyhow::Result;

use crate::compilation::do_statement_compiler::DoStatementCompiler;
use crate::compilation::if_statement_compiler::IfStatementCompiler;
use crate::compilation::let_statement_compiler::LetStatementCompiler;
use crate::compilation::return_statement_compiler::ReturnStatementCompiler;
use crate::compilation::while_statement_compiler::WhileStatementCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// statement = letStatement | ifStatement | whileStatement | doStatement | returnStatement
pub struct StatementCompiler {}

impl StatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        match KeyWord::from(tokenizer.peek()?.value())? {
            KeyWord::Let => {
                LetStatementCompiler::compile(tokenizer, writer, symbol_tables, written)?
            }
            KeyWord::If => IfStatementCompiler::compile(tokenizer, writer, symbol_tables, written)?,
            KeyWord::While => {
                WhileStatementCompiler::compile(tokenizer, writer, symbol_tables, written)?
            }
            KeyWord::Do => DoStatementCompiler::compile(tokenizer, writer, symbol_tables, written)?,
            KeyWord::Return => {
                ReturnStatementCompiler::compile(tokenizer, writer, symbol_tables, written)?
            }
            _ => {}
        }
        Ok(())
    }
}
