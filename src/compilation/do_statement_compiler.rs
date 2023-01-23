use std::io::Write;

use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
use anyhow::Result;

use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Do;

/// doStatement = ’do’ subroutineCall ’;’
pub struct DoStatementCompiler {}

impl DoStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <doStatement>
        writer.write_start_tag("doStatement", written)?;
        // do
        writer.write_key_word(tokenizer, vec![Do], written)?;
        // subroutineCall
        SubroutineCallCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </doStatement>
        writer.write_end_tag("doStatement", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::compilation::do_statement_compiler::DoStatementCompiler;
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
<doStatement>
  <keyword> do </keyword>
  <category> Class </category>
  <identifier> Output </identifier>
  <symbol> . </symbol>
  <category> Subroutine </category>
  <identifier> printString </identifier>
  <symbol> ( </symbol>
  <expressionList>
    <expression>
      <term>
        <stringConstant> THE AVERAGE IS:  </stringConstant>
      </term>
    </expression>
  </expressionList>
  <symbol> ) </symbol>
  <symbol> ; </symbol>
</doStatement>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "do Output.printString(\"THE AVERAGE IS: \");").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = DoStatementCompiler::compile(
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
