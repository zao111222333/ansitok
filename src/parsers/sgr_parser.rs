use crate::{parse::parse_visual_attribute, Output, VisualAttribute};

/// Creates a parser for Select Graphic Rendition(SGR) sequences.
pub fn parse_ansi_sgr(s: &str) -> SGRParser<'_> {
    SGRParser { text: Some(s) }
}

/// A parser for SGR sequences.
#[derive(Debug)]
pub struct SGRParser<'a> {
    text: Option<&'a str>,
}

impl<'a> Iterator for SGRParser<'a> {
    type Item = Output<'a, VisualAttribute>;

    fn next(&mut self) -> Option<Self::Item> {
        let origin = self.text?;
        if origin.is_empty() {
            return None;
        }

        let mut text = origin;
        if text.starts_with(';') && text.len() > 1 {
            text = &text[1..];
        }

        match parse_visual_attribute(text) {
            Ok((rest, mode)) => {
                self.text = Some(rest);
                return Some(Output::Escape(mode));
            }
            Err(_) => {
                self.text = None;
                return Some(Output::Text(origin));
            }
        }
    }
}
