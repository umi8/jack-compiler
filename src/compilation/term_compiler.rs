use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::{False, Null, This, True};
use crate::tokenizer::token_type::TokenType;
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
                    writer.write_key_word(tokenizer, vec![True, False, Null, This], written)?;
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
                "-" | "~" => {
                    // unaryOp
                    writer.write_symbol(tokenizer, written)?;
                    // term
                    TermCompiler::compile(tokenizer, writer, symbol_tables, written)?;
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
                    _ => writer.write_identifier(tokenizer, symbol_tables, written)?,
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
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::term_compiler::TermCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_int_const() {
        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "1").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
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
}
