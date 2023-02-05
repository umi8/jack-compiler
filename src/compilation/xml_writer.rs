use std::io::Write;

use anyhow::{bail, Error, Result};

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
