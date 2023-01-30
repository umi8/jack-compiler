use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_list_compiler::ExpressionListCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::vm_writer::VmWriter;

/// subroutineCall = subroutineName ’(’ expressionList ’)’ | (className | varName) ’.’ subroutineName ’(’ expressionList ’)’
pub struct SubroutineCallCompiler {}

impl SubroutineCallCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // subroutineName | (className | varName)
        tokenizer.advance()?;
        let subroutine_name = if tokenizer.peek()?.value() == "." {
            let class_name = String::from(tokenizer.identifier());

            // ’.’
            tokenizer.advance()?;

            // subroutineName
            tokenizer.advance()?;
            let subroutine_name = String::from(tokenizer.identifier());

            format!("{}.{}", class_name, subroutine_name)
        } else {
            String::from(tokenizer.identifier())
        };

        // ’(’
        tokenizer.advance()?;

        // expressionList
        let number_of_args =
            ExpressionListCompiler::compile(tokenizer, writer, symbol_tables, written)?;

        VmWriter::write_call(subroutine_name.as_str(), number_of_args, written)?;

        // ’)’
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, SeekFrom, Write};

    use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
    use crate::compilation::xml_writer::XmlWriter;
    use crate::symbol_table::symbol_tables::SymbolTables;
    use crate::tokenizer::jack_tokenizer::JackTokenizer;

    #[test]
    fn can_compile() {
        let expected = "\
push constant 100
call Output.printInt 1
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "Output.printInt(100)").unwrap();
        src_file.seek(SeekFrom::Start(0)).unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut writer = XmlWriter::new();
        let mut symbol_tables = SymbolTables::new();

        let result = SubroutineCallCompiler::compile(
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
