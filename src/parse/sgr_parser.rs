use super::{parsers::parse_visual_attribute, Output, VisualAttribute};

/// Creates a parser for Select Graphic Rendition(SGR) sequences.
pub fn parse_ansi_sgr(text: &str) -> SGRParser<'_> {
    SGRParser {
        text: Some(text),
        is_start: true,
    }
}

/// A parser for SGR sequences.
#[derive(Debug)]
pub struct SGRParser<'a> {
    is_start: bool,
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
        if !self.is_start && text.starts_with(';') && text.len() > 1 {
            text = &text[1..];
        }

        if self.is_start {
            self.is_start = false;
        }

        let attr = parse_visual_attribute(text);
        match attr {
            Ok((rest, mode)) => {
                // we need to check that next chars are either separator or it's an end of a string
                if !rest.is_empty() && !rest.starts_with(';') {
                    self.text = None;
                    Some(Output::Text(origin))
                } else {
                    self.text = Some(rest);
                    Some(Output::Escape(mode))
                }
            }
            Err(_) => {
                self.text = None;
                Some(Output::Text(origin))
            }
        }
    }
}
