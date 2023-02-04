use std::io::Write;

use anyhow::Result;

use crate::compilation::parameter_list_compiler::ParameterListCompiler;
use crate::compilation::subroutine_body_compiler::SubroutineBodyCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::vm_writer::VmWriter;

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

        let subroutine_name = {
            // ’constructor’ | ’function’ | ’method’
            tokenizer.advance()?;
            // ’void’ | type
            tokenizer.advance()?;
            // subroutineName
            tokenizer.advance()?;
            String::from(tokenizer.identifier())
        };

        // ’(’
        tokenizer.advance()?;
        // parameterList
        ParameterListCompiler::compile(tokenizer, symbol_tables)?;

        // ’)’
        tokenizer.advance()?;

        // subroutineBody
        SubroutineBodyCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        VmWriter::write_function(
            format!("{}.{}", class_name, subroutine_name).as_str(),
            symbol_tables.var_count(Kind::Var),
            written,
        )?;

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
function Test.convert 3
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "function void convert(int value) {{").unwrap();
        writeln!(src_file, "    var int mask, position;").unwrap();
        writeln!(src_file, "    var boolean loop;").unwrap();
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
