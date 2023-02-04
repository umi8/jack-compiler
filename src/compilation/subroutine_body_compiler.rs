use std::io::Write;

use anyhow::Result;

use crate::compilation::statements_compiler::StatementsCompiler;
use crate::compilation::var_dec_compiler::VarDecCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::key_word::KeyWord;
use crate::writer::vm_writer::VmWriter;

/// subroutineBody = ’{’ varDec* statements ’}’
pub struct SubroutineBodyCompiler {}

impl SubroutineBodyCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        subroutine_name: &str,
        written: &mut impl Write,
    ) -> Result<()> {
        // ’{’
        tokenizer.advance()?;

        // varDec*
        loop {
            if !KeyWord::exists(tokenizer.peek()?.value()) {
                break;
            }
            match KeyWord::from(tokenizer.peek()?.value())? {
                KeyWord::Var => VarDecCompiler::compile(tokenizer, symbol_tables)?,
                _ => break,
            }
        }

        if let Some(class_name) = symbol_tables.type_of("this") {
            VmWriter::write_function(
                format!("{}.{}", class_name, subroutine_name).as_str(),
                symbol_tables.var_count(Kind::Var),
                written,
            )?;
        }

        // statements
        StatementsCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        // ’}’
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::kind::Kind;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
function Test.convert 3
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "{{").unwrap();
        writeln!(src_file, "    var int mask, position;").unwrap();
        writeln!(src_file, "    var boolean loop;").unwrap();
        writeln!(src_file, "}}").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("this", "Test", &Kind::Argument);

        let result = SubroutineBodyCompiler::compile(
            &mut tokenizer,
            &mut writer,
            &mut symbol_tables,
            "convert",
            &mut output,
        );
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
