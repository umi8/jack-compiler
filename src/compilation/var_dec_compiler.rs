use anyhow::Result;

use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::token_type::TokenType::Symbol;

/// varDec = ’var’ type varName (’,’ varName)* ’;’
pub struct VarDecCompiler {}

impl VarDecCompiler {
    pub fn compile(tokenizer: &mut JackTokenizer, symbol_tables: &mut SymbolTables) -> Result<()> {
        // ’var’
        tokenizer.advance()?;

        // type
        let type_name = String::from(tokenizer.peek()?.value());
        tokenizer.advance()?;

        // varName
        let var_name = String::from(tokenizer.peek()?.value());
        symbol_tables.define(&var_name, &type_name, &Kind::Var);
        tokenizer.advance()?;

        // (’,’ varName)*
        loop {
            if tokenizer.peek()?.token_type() == &Symbol && tokenizer.peek()?.value() == "," {
                // ','
                tokenizer.advance()?;

                // varName
                let var_name = String::from(tokenizer.peek()?.value());
                symbol_tables.define(&var_name, &type_name, &Kind::Var);
                tokenizer.advance()?;
            } else {
                break;
            }
        }

        // ’;’
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::var_dec_compiler::VarDecCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "var int i, j, sum;").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();

        let result = VarDecCompiler::compile(&mut tokenizer, &mut symbol_tables);

        assert!(result.is_ok());
        assert_eq!(3, symbol_tables.var_count(Kind::Var));
    }
}
