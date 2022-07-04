use crate::{parse::parse_escape_sequence, AnsiSequence, Output};

/// Creates a parser for ANSI escape sequences.
pub fn parse_ansi(s: &str) -> AnsiSequenceParser<'_> {
    AnsiSequenceParser { text: s }
}

/// An ANSI escape sequence parser.
///
/// Which yields a token [Output] of either [AnsiSequence] or a string.
/// In case we don't recognize an ansi sequence we return a [Output::Text]
#[derive(Debug)]
pub struct AnsiSequenceParser<'a> {
    text: &'a str,
}

impl<'a> Iterator for AnsiSequenceParser<'a> {
    type Item = Output<'a, AnsiSequence<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.text.is_empty() {
            return None;
        }

        let res = parse_escape_sequence(self.text);
        match res {
            Ok((rest, escape)) => {
                self.text = rest;
                Some(Output::Escape(escape))
            }
            Err(_) => {
                let rest_text = self.text.strip_prefix('\u{1b}').unwrap_or(self.text);
                let pos = rest_text.find('\u{1b}');
                match pos {
                    Some(mut pos) => {
                        if self.text.len() > rest_text.len() {
                            pos += 1;
                        }

                        let temp = &self.text[..pos];
                        self.text = &self.text[pos..];
                        Some(Output::Text(temp))
                    }
                    None => {
                        let temp = self.text;
                        self.text = "";
                        Some(Output::Text(temp))
                    }
                }
            }
        }
    }
}
