use std::io::Write;

use anyhow::Result;

use crate::compilation::class_compiler::ClassCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

pub trait CompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self;
    fn compile(&mut self, writer: &mut impl Write) -> Result<()>;
}

pub struct XmlCompilationEngine {
    tokenizer: JackTokenizer,
    writer: XmlWriter,
    symbol_tables: SymbolTables,
}

impl CompilationEngine for XmlCompilationEngine {
    fn new(tokenizer: JackTokenizer) -> Self {
        XmlCompilationEngine {
            tokenizer,
            writer: XmlWriter::new(),
            symbol_tables: SymbolTables::new(),
        }
    }

    fn compile(&mut self, written: &mut impl Write) -> Result<()> {
        ClassCompiler::compile(
            &mut self.tokenizer,
            &mut self.writer,
            &mut self.symbol_tables,
            written,
        )?;
        Ok(())
    }
}
