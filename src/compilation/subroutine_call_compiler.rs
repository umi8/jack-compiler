use std::io::Write;

use anyhow::Result;

use crate::compilation::expression_list_compiler::ExpressionListCompiler;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::writer::segment::Segment;
use crate::writer::vm_writer::VmWriter;

/// subroutineCall = subroutineName ’(’ expressionList ’)’ | (className | varName) ’.’ subroutineName ’(’ expressionList ’)’
pub struct SubroutineCallCompiler {}

impl SubroutineCallCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        let mut number_of_args = 0;

        // subroutineName | (className | varName)
        tokenizer.advance()?;
        let subroutine_name = if tokenizer.peek()?.value() == "." {
            let var_class_name = String::from(tokenizer.identifier());

            if let Some(kind) = symbol_tables.kind_of(&var_class_name) {
                let segment = Segment::from(kind);
                let index = match kind {
                    Kind::Static | Kind::Field | Kind::Var => {
                        symbol_tables.index_of(&var_class_name).unwrap()
                    }
                    Kind::Argument => symbol_tables.index_of(&var_class_name).unwrap(),
                };
                VmWriter::write_push(&segment, index, written)?;
                number_of_args += 1;
            }

            let class_name = if symbol_tables.type_of(&var_class_name).is_some() {
                symbol_tables.type_of(&var_class_name).unwrap()
            } else {
                var_class_name
            };

            // ’.’
            tokenizer.advance()?;

            // subroutineName
            tokenizer.advance()?;
            let subroutine_name = String::from(tokenizer.identifier());

            format!("{class_name}.{subroutine_name}")
        } else {
            // In the case of a method,
            // pass a reference to the object to which the method belongs as the first argument to be pushed.
            VmWriter::write_push(&Segment::Pointer, 0, written)?;
            number_of_args += 1;

            let class_name = String::from(&symbol_tables.class_name);
            let subroutine_name = String::from(tokenizer.identifier());
            format!("{class_name}.{subroutine_name}")
        };

        // ’(’
        tokenizer.advance()?;

        // expressionList
        number_of_args += ExpressionListCompiler::compile(tokenizer, symbol_tables, written)?;

        VmWriter::write_call(subroutine_name.as_str(), number_of_args, written)?;

        // ’)’
        tokenizer.advance()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Seek, Write};

    use crate::compilation::subroutine_call_compiler::SubroutineCallCompiler;
    use crate::symbol_table::kind::Kind;
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
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();

        let result =
            SubroutineCallCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_compile_method() {
        let expected = "\
push pointer 0
push constant 100
call Output.printInt 2
"
        .to_string();

        let mut src_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(src_file, "printInt(100)").unwrap();
        src_file.rewind().unwrap();
        let path = src_file.path();
        let mut output = Vec::<u8>::new();

        let mut tokenizer = JackTokenizer::new(path).unwrap();
        let mut symbol_tables = SymbolTables::new();
        symbol_tables.define("this", "Output", &Kind::Argument);

        let result =
            SubroutineCallCompiler::compile(&mut tokenizer, &mut symbol_tables, &mut output);
        let actual = String::from_utf8(output).unwrap();

        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
