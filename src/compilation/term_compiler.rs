use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::token_type::TokenType;
use crate::writer::command::Command;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// term = integerConstant | stringConstant | keywordConstant | varName | varName ’[’ expression ’]’ | subroutineCall | ’(’ expression ’)’ | unaryOp term
pub struct TermCompiler {}

impl TermCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        match tokenizer.peek()?.token_type() {
            TokenType::Keyword => {
                if tokenizer.peek()?.is_keyword_constant()? {
                    tokenizer.advance()?;
                    match tokenizer.key_word()? {
                        KeyWord::True => {
                            VmWriter::write_push(&Segment::Constant, 1, written)?;
                            VmWriter::write_arithmetic(&Command::Neg, written)?;
                        }
                        KeyWord::False | KeyWord::Null => {
                            VmWriter::write_push(&Segment::Constant, 0, written)?
                        }
                        KeyWord::This => {
                            // Set the base of the object to the base of this segment
                            VmWriter::write_push(&Segment::Pointer, 0, written)?
                        }
                        _ => {}
                    }
                }
            }
            TokenType::Symbol => match tokenizer.peek()?.value().as_str() {
                "(" => {
                    // '('
                    tokenizer.advance()?;
                    // expression
                    ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
                    // ')'
                    tokenizer.advance()?;
                }
                "-" => {
                    // unaryOp
                    tokenizer.advance()?;
                    // term
                    TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;
                    VmWriter::write_arithmetic(&Command::Neg, written)?;
                }
                "~" => {
                    // unaryOp
                    tokenizer.advance()?;
                    // term
                    TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;
                    VmWriter::write_arithmetic(&Command::Not, written)?;
                }
                _ => {}
            },
            TokenType::Identifier => {
                match tokenizer.peek_second()?.value().as_str() {
                    "[" => {
                        // varName
                        tokenizer.advance()?;
                        let var_name = String::from(tokenizer.identifier());

                        if let Some(kind) = symbol_tables.kind_of(&var_name) {
                            let segment = Segment::from(kind);
                            let index = match kind {
                                Kind::Static | Kind::Field | Kind::Var => {
                                    symbol_tables.index_of(&var_name).unwrap()
                                }
                                Kind::Argument => symbol_tables.index_of(&var_name).unwrap() - 1,
                            };
                            VmWriter::write_push(&segment, index, written)?;
                        }

                        // '['
                        tokenizer.advance()?;
                        // expression
                        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
                        // ']'
                        tokenizer.advance()?;

                        // add base address and index
                        VmWriter::write_arithmetic(&Command::Add, written)?;

                        // Use that segment to access var_name[expression]
                        VmWriter::write_pop(&Segment::Pointer, 1, written)?;
                        VmWriter::write_push(&Segment::That, 0, written)?;
                    }
                    "." | "(" => {
                        SubroutineCallCompiler::compile(tokenizer, writer, symbol_tables, written)?
                    }
                    _ => {
                        // varName
                        tokenizer.advance()?;
                        let var_name = String::from(tokenizer.identifier());

                        if let Some(kind) = symbol_tables.kind_of(&var_name) {
                            let segment = Segment::from(kind);
                            let index = match kind {
                                Kind::Static | Kind::Field | Kind::Var => {
                                    symbol_tables.index_of(&var_name).unwrap()
                                }
                                Kind::Argument => symbol_tables.index_of(&var_name).unwrap() - 1,
                            };
                            VmWriter::write_push(&segment, index, written)?;
                        }
                    }
                }
            }
            TokenType::IntConst => {
                tokenizer.advance()?;
                VmWriter::write_push(&Segment::Constant, tokenizer.int_val()?, written)?;
            }
            TokenType::StringConst => {
                tokenizer.advance()?;
                let value = tokenizer.string_val();
                VmWriter::write_push(&Segment::Constant, value.len(), written)?;
                VmWriter::write_call("String.new", 1, written)?;
                for c in value.chars() {
                    let unicode_hex = format!("{:x}", c as u32);
                    VmWriter::write_push(
                        &Segment::Constant,
                        usize::from_str_radix(&unicode_hex, 16)?,
                        written,
                    )?;
                    VmWriter::write_call("String.appendChar", 2, written)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::term_compiler::TermCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_identifier() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "value & mask").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("this", "Test", &Kind::Argument);
        symbol_tables.define("value", "int", &Kind::Argument);

        let result =
            TermCompiler::compile(&mut tokenizer, &mut writer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!("push argument 0\n", actual);
    }

    #[test]
    fn can_compile_int_const() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "1").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result =
            TermCompiler::compile(&mut tokenizer, &mut writer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!("push constant 1\n", actual);
    }

    #[test]
    fn can_compile_string_const() {
        let expected = "\
push constant 5
call String.new 1
push constant 72
call String.appendChar 2
push constant 111
call String.appendChar 2
push constant 119
call String.appendChar 2
push constant 63
call String.appendChar 2
push constant 32
call String.appendChar 2
";
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "\"How? \"").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result =
            TermCompiler::compile(&mut tokenizer, &mut writer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_neg() {
        let expected = "\
push constant 1
neg
";
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "-1").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result =
            TermCompiler::compile(&mut tokenizer, &mut writer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_array() {
        let expected = "\
push local 0
push local 1
add
pop pointer 1
push that 0
";
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "a[i]").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("a", "Array", &Kind::Var);
        symbol_tables.define("i", "int", &Kind::Var);

        let result =
            TermCompiler::compile(&mut tokenizer, &mut writer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
