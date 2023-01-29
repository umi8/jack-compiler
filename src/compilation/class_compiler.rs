use std::io::Write;

use anyhow::Result;

use crate::compilation::class_var_dec_compiler::ClassVarDecCompiler;
use crate::compilation::subroutine_dec_compiler::SubroutineDecCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// class = ’class’ className ’{’ classVarDec* subroutineDec* ’}’
pub struct ClassCompiler {}

impl ClassCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’class’
        tokenizer.advance()?;

        // className
        let class_name = String::from(tokenizer.identifier());
        tokenizer.advance()?;

        // {
        tokenizer.advance()?;

        // classVarDec*
        loop {
            if !KeyWord::exists(tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Static | KeyWord::Field => {
                    ClassVarDecCompiler::compile(tokenizer, writer, symbol_tables, written)?
                }
                _ => break,
            }
        }

        // subroutineDec*
        loop {
            if !KeyWord::exists(tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Constructor | KeyWord::Function | KeyWord::Method => {
                    SubroutineDecCompiler::compile(
                        tokenizer,
                        writer,
                        symbol_tables,
                        &class_name,
                        written,
                    )?
                }
                _ => break,
            }
        }

        // }
        tokenizer.advance()?;

        Ok(())
    }
}
