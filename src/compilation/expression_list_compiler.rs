use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// expressionList = (expression (’,’ expression)* )?
pub struct ExpressionListCompiler {}

impl ExpressionListCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<usize> {
        let mut expression_count = 0;

        // (expression)?
        if tokenizer.is_term()? {
            // expression
            ExpressionCompiler::compile(tokenizer, symbol_tables, written)?;
            expression_count += 1;
        }

        // (’,’ expression)*
        while tokenizer.peek()?.value() == "," {
            // ’,’
            tokenizer.advance()?;

            // expression
            ExpressionCompiler::compile(tokenizer, symbol_tables, written)?;
            expression_count += 1;
        }

        Ok(expression_count)
    }
}
