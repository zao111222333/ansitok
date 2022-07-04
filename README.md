# ansi_rs

The library for parsing ANSI escape sequences.

The list of covered sequences.

* Cursor Position
* Cursor {Up, Down, Forward, Backward}
* Cursor {Save, Restore}
* Erase Display
* Erase Line
* Set Graphics mode
* Set/Reset Text Mode

# Usage

```rust
use ansi_rs::{parse_ansi, Output};

let text = "\x1b[31;1;4mHello World\x1b[0m";

for output in parse_ansi(text) {
    match output {
        Output::Text(text) => println!("Got a text: {:?}", text),
        Output::Escape(esc) => println!("Got an escape sequence: {:?}", esc),
    }
}
```

# `no_std` support

`no_std` is supported via disabling the `std` feature in your `Cargo.toml`.

# Notes

The project got an insiration from https://gitlab.com/davidbittner/ansi-parser.
