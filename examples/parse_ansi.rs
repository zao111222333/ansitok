use ansitok::{parse_ansi, Output};

fn main() {
    let text = "\x1b[31;1;4mHello World\x1b[0m";

    for output in parse_ansi(text) {
        match output {
            Output::Text(text) => println!("Got a text: {:?}", text),
            Output::Escape(esc) => println!("Got an escape sequence: {:?}", esc),
        }
    }
}
