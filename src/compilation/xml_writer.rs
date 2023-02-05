use std::io::Write;

use anyhow::{bail, Error, Result};

use crate::symbol_table::symbol_tables::SymbolTables;
use crate::tokenizer::jack_tokenizer::JackTokenizer;
use crate::tokenizer::token_type::TokenType;

pub struct XmlWriter {
    indent: String,
}

impl XmlWriter {
    pub fn new() -> Self {
        XmlWriter {
            indent: String::new(),
        }
    }

    pub fn write_identifier(
        &mut self,
        tokenizer: &mut JackTokenizer,
        symbol_tables: &mut SymbolTables,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::Identifier => {
                let var_name = tokenizer.identifier();

                // Category
                match symbol_tables.kind_of(var_name) {
                    None => {
                        if var_name.chars().collect::<Vec<char>>()[0].is_uppercase() {
                            writeln!(written, "{}<category> Class </category>", self.indent)?;
                        } else {
                            writeln!(written, "{}<category> Subroutine </category>", self.indent)?;
                        }
                    }
                    Some(kind) => {
                        writeln!(written, "{}<category> {} </category>", self.indent, kind)?;
                    }
                }

                // is_defined or used

                // kind
                if let Some(kind) = symbol_tables.kind_of(var_name) {
                    writeln!(written, "{}<kind> {} </kind>", self.indent, kind)?;
                }

                // execution number
                if let Some(index) = symbol_tables.index_of(var_name) {
                    writeln!(written, "{}<index> {} </index>", self.indent, index)?;
                }

                writeln!(
                    written,
                    "{}<identifier> {} </identifier>",
                    self.indent,
                    tokenizer.identifier()
                )?;
            }
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }

    pub fn write_symbol(
        &mut self,
        tokenizer: &mut JackTokenizer,
        written: &mut impl Write,
    ) -> Result<()> {
        tokenizer.advance()?;
        match tokenizer.token_type()? {
            TokenType::Symbol => {
                let symbol = match tokenizer.symbol() {
                    '<' => "&lt;",
                    '>' => "&gt;",
                    '&' => "&amp;",
                    _ => "",
                };

                if symbol.is_empty() {
                    writeln!(
                        written,
                        "{}<symbol> {} </symbol>",
                        self.indent,
                        tokenizer.symbol()
                    )?
                } else {
                    writeln!(written, "{}<symbol> {} </symbol>", self.indent, symbol)?
                }
            }
            _ => bail!(Error::msg("Illegal token")),
        }
        Ok(())
    }
}
