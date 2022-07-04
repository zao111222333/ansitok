use ansi_rs::{
    parse_ansi,
    {AnsiSequence::*, Output::Escape as esc, Output::Text as text},
};

macro_rules! test_parse_ansi {
    ($name:ident, $string:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let sequences: Vec<_> = parse_ansi($string).collect();
            assert_eq!(sequences, $expected);
        }
    };
}

test_parse_ansi!(empty, "", []);
test_parse_ansi!(
    parse_0,
    "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36m\x1b[1m-`",
    [
        esc(ResetMode(25)),
        esc(ResetMode(7)),
        esc(SelectGraphicRendition("0")),
        esc(SelectGraphicRendition("36")),
        esc(SelectGraphicRendition("1")),
        text("-`")
    ]
);
test_parse_ansi!(
    parse_1,
    "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36;1;15;2m\x1b[1m-`",
    [
        esc(ResetMode(25)),
        esc(ResetMode(7)),
        esc(SelectGraphicRendition("0")),
        esc(SelectGraphicRendition("36;1;15;2")),
        esc(SelectGraphicRendition("1")),
        text("-`")
    ]
);
test_parse_ansi!(
    parse_2,
    "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36;1;15;2m\x1b[1m-`",
    [
        esc(ResetMode(25)),
        esc(ResetMode(7)),
        esc(SelectGraphicRendition("0")),
        esc(SelectGraphicRendition("36;1;15;2")),
        esc(SelectGraphicRendition("1")),
        text("-`")
    ]
);
test_parse_ansi!(
    parse_3,
    "\x1b[=25l\x1b[=7l\x1b[0m\x1b[36;1;15;2;36;1;15;2m\x1b[1m-`",
    [
        esc(ResetMode(25)),
        esc(ResetMode(7)),
        esc(SelectGraphicRendition("0")),
        esc(SelectGraphicRendition("36;1;15;2;36;1;15;2")),
        esc(SelectGraphicRendition("1")),
        text("-`")
    ]
);
test_parse_ansi!(
    parse_4,
    "\x1b[H\x1b[123456H\x1b[;123456H\x1b[7asd;1234H\x1b[a;sd7H",
    [
        esc(CursorPos(1, 1)),
        esc(CursorPos(123456, 1)),
        esc(CursorPos(1, 123456)),
        text("\u{1b}[7asd;1234H"),
        text("\u{1b}[a;sd7H")
    ]
);
test_parse_ansi!(
    parse_5,
    "\x1b\x1b[33mFoobar",
    [esc(Escape), text("[33mFoobar")]
);
test_parse_ansi!(
    parse_6,
    "\x1b[38;5;45mFoobar\x1b[0m",
    [
        esc(SelectGraphicRendition("38;5;45")),
        text("Foobar"),
        esc(SelectGraphicRendition("0"))
    ]
);
