use anyhow::Result;

use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

pub struct ParameterListCompiler {}

impl ParameterListCompiler {
    pub fn compile(tokenizer: &mut JackTokenizer, symbol_tables: &mut SymbolTables) -> Result<()> {
        // ((type varName) (’,’ type varName)*)?
        if tokenizer.peek()?.is_type()? {
            // type
            let type_name = String::from(tokenizer.peek()?.value());
            tokenizer.advance()?;

            // varName
            let var_name = String::from(tokenizer.peek()?.value());
            symbol_tables.define(&var_name, &type_name, &Kind::Argument);
            tokenizer.advance()?;

            // (’,’ type varName)*
            while tokenizer.peek()?.value() == "," {
                // ’,’
                tokenizer.advance()?;

                // type
                let type_name = String::from(tokenizer.peek()?.value());
                tokenizer.advance()?;

                // varName
                let var_name = String::from(tokenizer.peek()?.value());
                symbol_tables.define(&var_name, &type_name, &Kind::Argument);
                tokenizer.advance()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::parameter_list_compiler::ParameterListCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "int count, boolean isTest, char c)").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();

        let result = ParameterListCompiler::compile(&mut tokenizer, &mut symbol_tables);

        assert!(result.is_ok());
        assert_eq!(3, symbol_tables.var_count(Kind::Argument));
    }
}
