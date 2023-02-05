use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::statements_compiler::StatementsCompiler;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::command::Command;
use crate::writer::label::LabelCreator;
use crate::writer::vm_writer::VmWriter;

/// whileStatement = ’while’ ’(’ expression ’)’ ’{’ statements ’}’
pub struct WhileStatementCompiler {}

impl WhileStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
        label_creator: &dyn LabelCreator,
    ) -> Result<()> {
        // while
        tokenizer.advance()?;
        // ’(’
        tokenizer.advance()?;

        let label_goto = label_creator.create("goto");
        VmWriter::write_label(&label_goto, written)?;

        // expression
        ExpressionCompiler::compile(tokenizer, symbol_tables, written)?;
        VmWriter::write_arithmetic(&Command::Not, written)?;

        let label_if: String = label_creator.create("if");
        VmWriter::write_if(&label_if, written)?;

        // ’)’
        tokenizer.advance()?;
        // ’{’
        tokenizer.advance()?;

        // statements
        StatementsCompiler::compile(tokenizer, symbol_tables, written)?;

        VmWriter::write_goto(&label_goto, written)?;
        VmWriter::write_label(&label_if, written)?;

        // ’}’
        tokenizer.advance()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use mockall::predicate::eq;

    use crate::compilation::while_statement_compiler::WhileStatementCompiler;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;
    use crate::writer::label::MockLabelCreator;

    #[test]
    fn can_compile() {
        let expected = "\
label goto_L2
push local 0
not
if-goto if_L1
push constant 8000
push local 1
add
push constant 1
call Memory.poke 2
pop temp 0
goto goto_L2
label if_L1
";

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "while (loop) {{").unwrap();
        writeln!(src_file, "    do Memory.poke(8000 + position, 1);").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("loop", "boolean", &Kind::Var);
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

        let result = WhileStatementCompiler::compile(
            &mut tokenizer,
            &mut symbol_tables,
            &mut output,
            &mock_label_creator,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
