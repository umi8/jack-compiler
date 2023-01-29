use anyhow::Result;

use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

/// classVarDec = (’static’ | ’field’) type varName (’,’ varName)* ’;’
pub struct ClassVarDecCompiler {}

impl ClassVarDecCompiler {
    pub fn compile(tokenizer: &mut JackTokenizer, symbol_tables: &mut SymbolTables) -> Result<()> {
        // static or field
        let kind = Kind::from_str(tokenizer.peek()?.value())?;
        tokenizer.advance()?;

        // type
        let type_name = String::from(tokenizer.peek()?.value());
        tokenizer.advance()?;

        // varName
        let var_name = String::from(tokenizer.peek()?.value());
        symbol_tables.define(&var_name, &type_name, &kind);
        tokenizer.advance()?;

        // (’,’ varName)*
        while tokenizer.peek()?.value() == "," {
            // ,
            tokenizer.advance()?;

            // varName
            let var_name = String::from(tokenizer.peek()?.value());
            symbol_tables.define(&var_name, &type_name, &kind);
            tokenizer.advance()?;
        }

        // ;
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::class_var_dec_compiler::ClassVarDecCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "static boolean isTest, isSomething;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();

        let result = ClassVarDecCompiler::compile(&mut tokenizer, &mut symbol_tables);

        assert!(result.is_ok());
        assert_eq!(2, symbol_tables.var_count(Kind::Static));
        assert_eq!(&Kind::Static, symbol_tables.kind_of("isTest").unwrap());
        assert_eq!("boolean", symbol_tables.type_of("isTest").unwrap());
        assert_eq!(1, symbol_tables.index_of("isSomething").unwrap());
    }
}
