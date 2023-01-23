use std::io::Write;

use anyhow::Result;

use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::var_dec_compiler::VarDecCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;

/// subroutineBody = ’{’ varDec* statements ’}’
pub struct SubroutineBodyCompiler {}

impl SubroutineBodyCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <subroutineBody>
        writer.write_start_tag("subroutineBody", written)?;
        // ’{’
        writer.write_symbol(tokenizer, written)?;
        // varDec*
        loop {
            if !KeyWord::exists(tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Var => VarDecCompiler::compile(tokenizer, writer, symbol_tables, written)?,
                _ => break,
            }
        }
        // statements
        StatementsCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // ’}’
        writer.write_symbol(tokenizer, written)?;
        // </subroutineBody>
        writer.write_end_tag("subroutineBody", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
<subroutineBody>
  <symbol> { </symbol>
  <varDec>
    <keyword> var </keyword>
    <category> Class </category>
    <identifier> Array </identifier>
    <category> Var </category>
    <kind> Var </kind>
    <index> 0 </index>
    <identifier> a </identifier>
    <symbol> ; </symbol>
  </varDec>
  <varDec>
    <keyword> var </keyword>
    <keyword> int </keyword>
    <category> Var </category>
    <kind> Var </kind>
    <index> 1 </index>
    <identifier> length </identifier>
    <symbol> ; </symbol>
  </varDec>
  <statements>
    <returnStatement>
      <keyword> return </keyword>
      <symbol> ; </symbol>
    </returnStatement>
  </statements>
  <symbol> } </symbol>
</subroutineBody>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "{{").unwrap();
        writeln!(src_file, "var Array a;").unwrap();
        writeln!(src_file, "var int length;").unwrap();
        writeln!(src_file, "return;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = SubroutineBodyCompiler::compile(
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