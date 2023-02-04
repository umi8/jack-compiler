use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::token_type::TokenType::Keyword;
use crate::writer::command::Command;
use crate::writer::label::LabelCreator;
use crate::writer::vm_writer::VmWriter;

/// ifStatement = ’if’ ’(’ expression ’)’ ’{’ statements ’}’ (’else’ ’{’ statements ’}’)?
pub struct IfStatementCompiler {}

impl IfStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
        label_creator: &dyn LabelCreator,
    ) -> Result<()> {
        // if
        tokenizer.advance()?;

        // ’(’
        tokenizer.advance()?;

        // expression
        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        VmWriter::write_arithmetic(&Command::Not, written)?;

        let label_if: String = label_creator.create("if");
        VmWriter::write_if(&label_if, written)?;

        // ’)’
        tokenizer.advance()?;

        // ’{’
        tokenizer.advance()?;

        // statements
        StatementsCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // ’}’
        tokenizer.advance()?;

        // (’else’ ’{’ statements ’}’)?
        if tokenizer.peek()?.token_type() == &Keyword
            && KeyWord::from(tokenizer.peek()?.value())? == KeyWord::Else
        {
            let label_goto = label_creator.create("goto");
            VmWriter::write_goto(&label_goto, written)?;
            VmWriter::write_label(&label_if, written)?;
            // else
            tokenizer.advance()?;
            // ’{’
            tokenizer.advance()?;
            // statements
            StatementsCompiler::compile(tokenizer, writer, symbol_tables, written)?;
            // ’}’
            tokenizer.advance()?;
            VmWriter::write_label(&label_goto, written)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::if_statement_compiler::IfStatementCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;
    use crate::writer::label::MockLabelCreator;

    #[test]
    fn can_compile() {
        let expected = "\
push argument 0
push local 0
and
push constant 0
eq
not
not
if-goto if_L1
push constant 8000
push local 1
add
push constant 1
call Memory.poke 2
goto goto_L2
label if_L1
push constant 8000
push local 1
add
push constant 0
call Memory.poke 2
label goto_L2
";

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "if (~((value & mask) = 0)) {{").unwrap();
        writeln!(src_file, "    do Memory.poke(8000 + position, 1);").unwrap();
        writeln!(src_file, "}}").unwrap();
        writeln!(src_file, "else {{").unwrap();
        writeln!(src_file, "    do Memory.poke(8000 + position, 0);").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("this", "Main", &Kind::Argument);
        symbol_tables.define("value", "int", &Kind::Argument);
        symbol_tables.define("mask", "int", &Kind::Var);
        symbol_tables.define("position", "int", &Kind::Var);

        let mut mock_label_creator = MockLabelCreator::default();
        mock_label_creator
            .expect_create()
            .with(eq("if"))
            .return_const(String::from("if_L1"));
        mock_label_creator
            .expect_create()
            .with(eq("goto"))
            .return_const(String::from("goto_L2"));

        let result = IfStatementCompiler::compile(
            &mut tokenizer,
            &mut writer,
            &mut symbol_tables,
            &mut output,
            &mock_label_creator,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
