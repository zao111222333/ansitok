use ansitok::{parse_ansi, parse_ansi_sgr, AnsiSequence, Output};

fn main() {
    let text = "\x1b[31;1;4mHello World\x1b[0m \x1b[38;2;255;255;0m!!!\x1b[0m";

    for output in parse_ansi(text) {
        if let Output::Escape(AnsiSequence::SelectGraphicRendition(sgr)) = output {
            for style in parse_ansi_sgr(sgr) {
                println!("{:?}", style);
            }
        }
    }
}
