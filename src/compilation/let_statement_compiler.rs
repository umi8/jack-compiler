use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_compiler::ExpressionCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord::Let;

/// letStatement = ’let’ varName (’[’ expression ’]’)? ’=’ expression ’;’
pub struct LetStatementCompiler {}

impl LetStatementCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <letStatement>
        writer.write_start_tag("letStatement", written)?;
        // let
        writer.write_key_word(tokenizer, vec![Let], written)?;
        // varName
        writer.write_identifier(tokenizer, symbol_tables, written)?;
        // (’[’ expression ’]’)?
        if tokenizer.peek()?.value() == "[" {
            // ’[’
            writer.write_symbol(tokenizer, written)?;
            // expression
            ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
            // ’]’
            writer.write_symbol(tokenizer, written)?;
        }
        // ’=’
        writer.write_symbol(tokenizer, written)?;
        // expression
        ExpressionCompiler::compile(tokenizer, writer, symbol_tables, written)?;
        // ’;’
        writer.write_symbol(tokenizer, written)?;
        // </letStatement>
        writer.write_end_tag("letStatement", written)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::let_statement_compiler::LetStatementCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
<letStatement>
  <keyword> let </keyword>
  <category> Var </category>
  <kind> Var </kind>
  <index> 0 </index>
  <identifier> length </identifier>
  <symbol> = </symbol>
  <expression>
    <term>
      <category> Class </category>
      <identifier> Keyboard </identifier>
      <symbol> . </symbol>
      <category> Subroutine </category>
      <identifier> readInt </identifier>
      <symbol> ( </symbol>
      <expressionList>
        <expression>
          <term>
            <stringConstant> HOW MANY NUMBERS?  </stringConstant>
          </term>
        </expression>
      </expressionList>
      <symbol> ) </symbol>
    </term>
  </expression>
  <symbol> ; </symbol>
</letStatement>
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            src_file,
            "let length = Keyboard.readInt(\"HOW MANY NUMBERS? \");"
        )
        .unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("length", "int", &Kind::Var);

        let result = LetStatementCompiler::compile(
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
