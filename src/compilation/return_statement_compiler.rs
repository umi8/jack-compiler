use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Return;

/// returnStatement = ’return’ expression? ’;’
pub struct ReturnStatementCompiler {}

impl ReturnStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <returnStatement>
        writer.write_start_tag("returnStatement", written)?;
        // return
        writer.write_key_word(tokenizer, vec![Return], written)?;
        // expression?
        if tokenizer.peek()?.value() != ";" {
            ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        }
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </returnStatement>
        writer.write_end_tag("returnStatement", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::return_statement_compiler::ReturnStatementCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile_return_statement() {
        let expected = "\
<returnStatement>
  <keyword> return </keyword>
  <expression>
    <term>
      <category> Var </category>
      <kind> Var </kind>
      <index> 0 </index>
      <identifier> x </identifier>
    </term>
  </expression>
  <symbol> ; </symbol>
</returnStatement>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "return x;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("x", "int", &Kind::Var);

        let result = ReturnStatementCompiler::compile(
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
