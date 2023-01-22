use std::io::Write;

use anyhow::Result;

use crate::compilation::type_compiler::TypeCompiler;
use crate::compilation::xml_writer::XmlWriter;
use crate::symbol_table::kind::Kind;
use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;

pub struct ParameterListCompiler {}

impl ParameterListCompiler {
    pub fn compile(
        tokenizer: &mut JackTokenizer,
        writer: &mut XmlWriter,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        // <parameterList>
        writer.write_start_tag("parameterList", written)?;

        // ((type varName) (’,’ type varName)*)?
        if tokenizer.peek()?.is_type()? {
            // type
            let type_name = String::from(tokenizer.peek()?.value());
            TypeCompiler::compile(tokenizer, writer, symbol_tables, written)?;

            // varName
            let var_name = String::from(tokenizer.peek()?.value());
            symbol_tables.define(&var_name, &type_name, &Kind::Argument);
            writer.write_identifier(tokenizer, symbol_tables, written)?;

            // (’,’ type varName)*
            while tokenizer.peek()?.value() == "," {
                // ’,’
                writer.write_symbol(tokenizer, written)?;

                // type
                let type_name = String::from(tokenizer.peek()?.value());
                TypeCompiler::compile(tokenizer, writer, symbol_tables, written)?;

                // varName
                let var_name = String::from(tokenizer.peek()?.value());
                symbol_tables.define(&var_name, &type_name, &Kind::Argument);
                writer.write_identifier(tokenizer, symbol_tables, written)?;
            }
        }

        // </parameterList>
        writer.write_end_tag("parameterList", written)?;
        Ok(())
    }
}
