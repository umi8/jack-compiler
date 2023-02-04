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
                        writer.write_identifier(tokenizer, symbol_tables, written)?;
                        // '['
                        writer.write_symbol(tokenizer, written)?;
                        // expression
                        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
                        // ']'
                        writer.write_symbol(tokenizer, written)?;
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
            TokenType::StringConst => writer.write_string_constant(tokenizer, written)?,
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
}
