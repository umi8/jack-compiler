use std::io::Write;

use anyhow::Result;

use crate::compilation::parameter_list_compiler::ParameterListCompiler;
use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
use crate::compilation::type_compiler::TypeCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::tokenizer::key_word::KeyWord::{Constructor, Function, Method, Void};
use crate::tokenizer::token_type::TokenType::Keyword;

/// subroutineDec =(’constructor’ | ’function’ | ’method’) (’void’ | type) subroutineName ’(’ parameterList ’)’ subroutineBody
pub struct SubroutineDecCompiler {}

impl SubroutineDecCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        class_name: &str,
        written: &mut impl Write,
    ) -> Result<()> {
        symbol_tables.start_subroutine();
        symbol_tables.define("this", class_name, &Kind::Argument);

        // <subroutineDec>
        writer.write_start_tag("subroutineDec", written)?;

        // ’constructor’ | ’function’ | ’method’
        writer.write_key_word(tokenizer, vec![Constructor, Function, Method], written)?;

        // ’void’ | type
        if tokenizer.peek()?.token_type() == &Keyword
            && KeyWord::from(tokenizer.peek()?.value())? == Void
        {
            writer.write_key_word(tokenizer, vec![Void], written)?
        } else {
            TypeCompiler::compile(tokenizer, writer, written)?
        }

        // subroutineName
        writer.write_identifier(tokenizer, written)?;

        // ’(’
        writer.write_symbol(tokenizer, written)?;

        // parameterList
        ParameterListCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // ’)’
        writer.write_symbol(tokenizer, written)?;

        // subroutineBody
        SubroutineBodyCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // </subroutineDec>
        writer.write_end_tag("subroutineDec", written)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::subroutine_dec_compiler::SubroutineDecCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
<subroutineDec>
  <keyword> function </keyword>
  <keyword> void </keyword>
  <identifier> main </identifier>
  <symbol> ( </symbol>
  <parameterList>
  </parameterList>
  <symbol> ) </symbol>
  <subroutineBody>
    <symbol> { </symbol>
    <statements>
      <returnStatement>
        <keyword> return </keyword>
        <symbol> ; </symbol>
      </returnStatement>
    </statements>
    <symbol> } </symbol>
  </subroutineBody>
</subroutineDec>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "function void main() {{").unwrap();
        writeln!(src_file, "return;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = SubroutineDecCompiler::compile(
            &mut tokenizer,
            &mut writer,
            &mut symbol_tables,
            "Test",
            &mut output,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
