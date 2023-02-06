use std::io::Write;

use anyhow::Result;

use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::var_dec_compiler::VarDecCompiler;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// subroutineBody = ’{’ varDec* statements ’}’
pub struct SubroutineBodyCompiler {}

impl SubroutineBodyCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        subroutine_name: &str,
        subroutine_type: &str,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’{’
        tokenizer.advance()?;

        // varDec*
        while KeyWord::exists(tokenizer.peek()?.value())
            && KeyWord::from(tokenizer.peek()?.value())? == KeyWord::Var
        {
            VarDecCompiler::compile(tokenizer, symbol_tables)?
        }

        VmWriter::write_function(
            format!("{}.{}", symbol_tables.class_name, subroutine_name).as_str(),
            symbol_tables.var_count(Kind::Var),
            written,
        )?;

        Self::set_pointer(symbol_tables, subroutine_type, written)?;

        // statements
        StatementsCompiler::compile(tokenizer, symbol_tables, written)?;

        // ’}’
        tokenizer.advance()?;

        Ok(())
    }

    fn set_pointer(
        symbol_tables: &mut SymbolTables,
        subroutine_type: &str,
        written: &mut impl Write,
    ) -> Result<()> {
        match subroutine_type {
            "constructor" => {
                VmWriter::write_push(
                    &Segment::Constant,
                    symbol_tables.var_count(Kind::Field),
                    written,
                )?;
                // Allocate as much memory as the number of FIELD for the new object
                VmWriter::write_call("Memory.alloc", 1, written)?;
                // Set this segment to point to the current object (constructor and method only)
                VmWriter::write_pop(&Segment::Pointer, 0, written)?;
            }
            "method" => {
                VmWriter::write_push(&Segment::Argument, 0, written)?;
                // Set this segment to point to the current object (constructor and method only)
                VmWriter::write_pop(&Segment::Pointer, 0, written)?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_function() {
        let expected = "\
function Test.convert 3
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "{{").unwrap();
        writeln!(src_file, "    var int mask, position;").unwrap();
        writeln!(src_file, "    var boolean loop;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.class_name = String::from("Test");

        let result = SubroutineBodyCompiler::compile(
            &mut tokenizer,
            &mut symbol_tables,
            "convert",
            "function",
            &mut output,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_constructor() {
        let expected = "\
function SquareGame.new 0
push constant 2
call Memory.alloc 1
pop pointer 0
push constant 0
push constant 0
push constant 30
call Square.new 3
pop this 0
push constant 0
pop this 1
push pointer 0
return
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "{{").unwrap();
        writeln!(src_file, "    let square = Square.new(0, 0, 30);").unwrap();
        writeln!(src_file, "    let direction = 0;").unwrap();
        writeln!(src_file, "    return this;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.class_name = String::from("SquareGame");
        symbol_tables.define("square", "Square", &Kind::Field);
        symbol_tables.define("direction", "int", &Kind::Field);

        let result = SubroutineBodyCompiler::compile(
            &mut tokenizer,
            &mut symbol_tables,
            "new",
            "constructor",
            &mut output,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
