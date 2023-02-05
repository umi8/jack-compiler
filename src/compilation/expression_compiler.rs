use std::io::Write;

use anyhow::Result;

use crate::compilation::term_compiler::TermCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::command::Command;
use crate::writer::vm_writer::VmWriter;

/// expression = term (op term)*
pub struct ExpressionCompiler {}

impl ExpressionCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // term
        TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // (op term)*
        loop {
            if tokenizer.peek()?.is_op() {
                // op
                tokenizer.advance()?;
                let op = tokenizer.symbol();

                // term
                TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;

                if let Some(command) = Command::from(op) {
                    VmWriter::write_arithmetic(&command, written)?;
                } else if op == '*' {
                    VmWriter::write_call("Math.multiply", 2, written)?;
                } else {
                    // in case of '/'(divide)
                    VmWriter::write_call("Math.divide", 2, written)?;
                }
            } else {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::expression_compiler::ExpressionCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
push constant 1
push constant 2
push constant 3
call Math.multiply 2
add
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "1 + (2 * 3)").unwrap();
        writeln!(src_file, ")").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = ExpressionCompiler::compile(
            &mut tokenizer,
            &mut writer,
            &mut symbol_tables,
            &mut output,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
