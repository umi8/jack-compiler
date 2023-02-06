use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::command::Command;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// letStatement = ’let’ varName (’[’ expression ’]’)? ’=’ expression ’;’
pub struct LetStatementCompiler {}

impl LetStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // let
        tokenizer.advance()?;

        // varName
        tokenizer.advance()?;
        let var_name = String::from(tokenizer.identifier());

        // (’[’ expression ’]’)?
        let mut is_array = false;
        if tokenizer.peek()?.value() == "[" {
            is_array = true;

            if let Some(kind) = symbol_tables.kind_of(&var_name) {
                let segment = Segment::from(kind);
                let index = match kind {
                    Kind::Static | Kind::Field | Kind::Var => {
                        symbol_tables.index_of(&var_name).unwrap()
                    }
                    Kind::Argument => symbol_tables.index_of(&var_name).unwrap(),
                };
                VmWriter::write_push(&segment, index, written)?;
            }

            // ’[’
            tokenizer.advance()?;
            // expression
            ExpressionCompiler::compile(tokenizer, symbol_tables, written)?;
            // ’]’
            tokenizer.advance()?;

            // add base address and index
            VmWriter::write_arithmetic(&Command::Add, written)?;
        }

        // ’=’
        tokenizer.advance()?;

        // expression
        ExpressionCompiler::compile(tokenizer, symbol_tables, written)?;

        if is_array {
            // Set the that segment to point to the address of an array element (using "pointer 1")
            VmWriter::write_pop(&Segment::Temp, 0, written)?;
            VmWriter::write_pop(&Segment::Pointer, 1, written)?;
            // and access that array element using a "that 0" reference.
            VmWriter::write_push(&Segment::Temp, 0, written)?;
            VmWriter::write_pop(&Segment::That, 0, written)?;
        } else if let Some(kind) = symbol_tables.kind_of(&var_name) {
            let segment = Segment::from(kind);
            let index = match kind {
                Kind::Static | Kind::Field | Kind::Var => {
                    symbol_tables.index_of(&var_name).unwrap()
                }
                Kind::Argument => symbol_tables.index_of(&var_name).unwrap(),
            };
            VmWriter::write_pop(&segment, index, written)?;
        }

        // ’;’
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::let_statement_compiler::LetStatementCompiler;
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
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("value", "int", &Kind::Var);

        let result = LetStatementCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_array() {
        let expected = "\
push local 2
push local 0
push local 1
add
pop pointer 1
push that 0
add
pop local 2
";

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "let sum = sum + a[i];").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("a", "Array", &Kind::Var);
        symbol_tables.define("i", "int", &Kind::Var);
        symbol_tables.define("sum", "int", &Kind::Var);

        let result = LetStatementCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
