use std::io::Write;

use anyhow::Result;

use crate::compilation::class_compiler::ClassCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

pub struct CompilationEngine {
    tokenizer: JackTokenizer,
    symbol_tables: SymbolTables,
}

impl CompilationEngine {
    pub fn new(tokenizer: JackTokenizer) -> Self {
        CompilationEngine {
            tokenizer,
            symbol_tables: SymbolTables::new(),
        }
    }

    pub fn compile(&mut self, written: &mut impl Write) -> Result<()> {
        ClassCompiler::compile(&mut self.tokenizer, &mut self.symbol_tables, written)?;
        Ok(())
    }
}
