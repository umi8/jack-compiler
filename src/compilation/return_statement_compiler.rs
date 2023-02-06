use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// returnStatement = ’return’ expression? ’;’
pub struct ReturnStatementCompiler {}

impl ReturnStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // return
        tokenizer.advance()?;

        // expression?
        if tokenizer.peek()?.value() != ";" {
            ExpressionCompiler::compile(tokenizer, symbol_tables, written)?;
        } else {
            VmWriter::write_push(&Segment::Constant, 0, written)?;
        }

        // ’;’
        tokenizer.advance()?;

        VmWriter::write_return(written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::return_statement_compiler::ReturnStatementCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
push argument 1
push constant 2
call Math.multiply 2
return
"
        .to_string();
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "return mask * 2;").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("this", "Test", &Kind::Argument);
        symbol_tables.define("mask", "int", &Kind::Argument);

        let result =
            ReturnStatementCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_void() {
        let expected = "\
push constant 0
return
"
        .to_string();
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "return;").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();

        let result =
            ReturnStatementCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
