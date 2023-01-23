use std::io::Write;

use anyhow::Result;

use crate::compilation::type_compiler::TypeCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::{Field, Static};

/// classVarDec = (’static’ | ’field’) type varName (’,’ varName)* ’;’
pub struct ClassVarDecCompiler {}

impl ClassVarDecCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <classVarDec>
        writer.write_start_tag("classVarDec", written)?;

        // static or field
        let kind = Kind::from_str(tokenizer.peek()?.value())?;
        writer.write_key_word(tokenizer, vec![Static, Field], written)?;

        // type
        let type_name = String::from(tokenizer.peek()?.value());
        TypeCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // varName
        let var_name = String::from(tokenizer.peek()?.value());
        symbol_tables.define(&var_name, &type_name, &kind);
        writer.write_identifier(tokenizer, symbol_tables, written)?;

        // (’,’ varName)*
        while tokenizer.peek()?.value() == "," {
            // ,
            writer.write_symbol(tokenizer, written)?;

            // varName
            let var_name = String::from(tokenizer.peek()?.value());
            symbol_tables.define(&var_name, &type_name, &kind);
            writer.write_identifier(tokenizer, symbol_tables, written)?;
        }

        // ;
        writer.write_symbol(tokenizer, written)?;

        // </classVarDec>
        writer.write_end_tag("classVarDec", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::class_var_dec_compiler::ClassVarDecCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
<classVarDec>
  <keyword> static </keyword>
  <keyword> boolean </keyword>
  <category> Static </category>
  <kind> Static </kind>
  <index> 0 </index>
  <identifier> test </identifier>
  <symbol> ; </symbol>
</classVarDec>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "static boolean test;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = ClassVarDecCompiler::compile(
            &mut tokenizer,
            &mut writer,
            &mut symbol_tables,
            &mut output,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_multiple_variants() {
        let expected = "\
<classVarDec>
  <keyword> static </keyword>
  <keyword> boolean </keyword>
  <category> Static </category>
  <kind> Static </kind>
  <index> 0 </index>
  <identifier> test </identifier>
  <symbol> , </symbol>
  <category> Static </category>
  <kind> Static </kind>
  <index> 1 </index>
  <identifier> hoge </identifier>
  <symbol> ; </symbol>
</classVarDec>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "static boolean test, hoge;").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = ClassVarDecCompiler::compile(
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
