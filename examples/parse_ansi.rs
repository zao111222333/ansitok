use std::borrow::Cow;

use ansitok_forked::parse_ansi;

fn main() {
    let text = "\x1b[31;1;4mHello World\x1b[0m";

    for output in parse_ansi(Cow::Borrowed(text)) {
        println!("Got a text: {:?}", output)
    }
}
