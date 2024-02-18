use std::ops::Range;

use ansitok::{parse_ansi, ElementKind};
use nom::Slice;

fn main() {
    let text = "\x1b[31;1;4mHello World\x1b[0m";

    for output in parse_ansi(text) {
        match output.kind() {
            ElementKind::Text => {
                println!(
                    "Got a text: {:?}",
                    text.slice(Range {
                        start: output.start(),
                        end: output.end()
                    })
                );
            }
            _ => {
                println!(
                    "Got an escape sequence: {:?} from {:#?} to {:#?}",
                    output.kind(),
                    output.start(),
                    output.end()
                );
            }
        }
    }
}
