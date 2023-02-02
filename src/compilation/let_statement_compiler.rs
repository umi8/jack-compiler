use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// letStatement = ’let’ varName (’[’ expression ’]’)? ’=’ expression ’;’
pub struct LetStatementCompiler {}

impl LetStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // let
        tokenizer.advance()?;

        // varName
        tokenizer.advance()?;
        let var_name = String::from(tokenizer.identifier());

        // (’[’ expression ’]’)?
        if tokenizer.peek()?.value() == "[" {
            // ’[’
            writer.write_symbol(tokenizer, written)?;
            // expression
            ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
            // ’]’
            writer.write_symbol(tokenizer, written)?;
        }

        // ’=’
        tokenizer.advance()?;

        // expression
        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        if let Some(index) = symbol_tables.index_of(&var_name) {
            let kind = symbol_tables.kind_of(&var_name).unwrap();
            let segment = Segment::from(kind);
            VmWriter::write_pop(&segment, index, written)?;
        }

        // ’;’
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::let_statement_compiler::LetStatementCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
push constant 8000
call Memory.peek 1
pop local 0
";

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "let value = Memory.peek(8000);").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("value", "int", &Kind::Var);

        let result = LetStatementCompiler::compile(
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
