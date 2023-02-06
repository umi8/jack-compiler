use std::io::Write;

use anyhow::Result;

use crate::compilation::class_var_dec_compiler::ClassVarDecCompiler;
use crate::compilation::subroutine_dec_compiler::SubroutineDecCompiler;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// class = ’class’ className ’{’ classVarDec* subroutineDec* ’}’
pub struct ClassCompiler {}

impl ClassCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’class’
        tokenizer.advance()?;

        // className
        tokenizer.advance()?;
        let class_name = String::from(tokenizer.identifier());
        symbol_tables.class_name = String::from(&class_name);

        // {
        tokenizer.advance()?;

        // classVarDec*
        while Self::exist_class_var_dec(tokenizer)? {
            ClassVarDecCompiler::compile(tokenizer, symbol_tables)?;
        }

        // subroutineDec*
        while KeyWord::exists(tokenizer.peek()?.value()) {
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Constructor | KeyWord::Function => {
                    symbol_tables.start_subroutine();
                    SubroutineDecCompiler::compile(tokenizer, symbol_tables, written)?;
                }
                KeyWord::Method => {
                    symbol_tables.start_subroutine();
                    symbol_tables.define("this", &class_name, &Kind::Argument);
                    SubroutineDecCompiler::compile(tokenizer, symbol_tables, written)?;
                }
                _ => break,
            }
        }

        // }
        tokenizer.advance()?;

        Ok(())
    }

    fn exist_class_var_dec(tokenizer: &JackTokenizer) -> Result<bool> {
        if !KeyWord::exists(tokenizer.peek()?.value()) {
            return Ok(false);
        }
        match KeyWord::from(tokenizer.peek()?.value())? {
            KeyWord::Static | KeyWord::Field => Ok(true),
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::class_compiler::ClassCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "class Main {{").unwrap();
        writeln!(src_file, "    method void main() {{").unwrap();
        writeln!(src_file, "    }}").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();

        let result = ClassCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);

        assert!(result.is_ok());
        assert_eq!(&Kind::Argument, symbol_tables.kind_of("this").unwrap());
        assert_eq!("Main", symbol_tables.type_of("this").unwrap());
        assert_eq!(0, symbol_tables.index_of("this").unwrap());
    }
}
