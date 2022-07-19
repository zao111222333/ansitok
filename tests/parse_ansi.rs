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
    parse_escape,
    "\x1b\x1b\x1b\x1b\x1b",
    [esc(Escape), esc(Escape), text("\u{1b}")]
);
test_parse_ansi!(cur_pos_1, "\x1b[32;102H", [esc(CursorPos(32, 102))]);
test_parse_ansi!(cur_pos_2, "\x1b[32;102f", [esc(CursorPos(32, 102))]);
test_parse_ansi!(cur_pos_3, "\x1b[32;102;H", [text("\x1b[32;102;H")]);
test_parse_ansi!(cur_pos_4, "\x1b[32;102;f", [text("\x1b[32;102;f")]);
test_parse_ansi!(
    cur_pos_5,
    "\x1b[467434;3332H",
    [esc(CursorPos(467434, 3332))]
);
test_parse_ansi!(
    cur_pos_6,
    "\x1b[467434;3332f",
    [esc(CursorPos(467434, 3332))]
);
test_parse_ansi!(cur_pos_7, "\x1b[23;f", [esc(CursorPos(23, 1))]);
test_parse_ansi!(cur_pos_8, "\x1b[;23f", [esc(CursorPos(1, 23))]);
test_parse_ansi!(cur_pos_empty_1, "\x1b[f", [esc(CursorPos(1, 1))]);
test_parse_ansi!(cur_pos_empty_2, "\x1b[H", [esc(CursorPos(1, 1))]);
test_parse_ansi!(cur_pos_up, "\x1b[100A", [esc(CursorUp(100))]);
test_parse_ansi!(cur_pos_up_big, "\x1b[123213A", [esc(CursorUp(123213))]);
test_parse_ansi!(cur_pos_up_empty, "\x1b[A", [esc(CursorUp(1))]);
test_parse_ansi!(cur_pos_down, "\x1b[100B", [esc(CursorDown(100))]);
test_parse_ansi!(cur_pos_down_big, "\x1b[123213B", [esc(CursorDown(123213))]);
test_parse_ansi!(cur_pos_down_empty, "\x1b[B", [esc(CursorDown(1))]);
test_parse_ansi!(cur_pos_forward, "\x1b[100C", [esc(CursorForward(100))]);
test_parse_ansi!(
    cur_pos_forward_1,
    "\x1b[123213C",
    [esc(CursorForward(123213))]
);
test_parse_ansi!(cur_pos_forward_empty, "\x1b[C", [esc(CursorForward(1))]);
test_parse_ansi!(cur_pos_backward, "\x1b[100D", [esc(CursorBackward(100))]);
test_parse_ansi!(
    cur_pos_backward_1,
    "\x1b[123213D",
    [esc(CursorBackward(123213))]
);
test_parse_ansi!(cur_pos_backward_empty, "\x1b[D", [esc(CursorBackward(1))]);
test_parse_ansi!(set_mode, "\x1b[=23h", [esc(SetMode(23))]);
test_parse_ansi!(set_mode_1, "\x1b[=h", [text("\u{1b}[=h")]);
test_parse_ansi!(set_mode_2, "\x1b[=512h", [text("\u{1b}[=512h")]);
test_parse_ansi!(reset_mode, "\x1b[=23l", [esc(ResetMode(23))]);
test_parse_ansi!(reset_mode_1, "\x1b[=l", [text("\u{1b}[=l")]);
test_parse_ansi!(reset_mode_2, "\x1b[=512l", [text("\u{1b}[=512l")]);
test_parse_ansi!(set_top_bot, "\x1b[1;43r", [esc(SetTopAndBottom(1, 43))]);
test_parse_ansi!(set_top_bot_1, "\x1b[;43r", [esc(SetTopAndBottom(1, 43))]);
test_parse_ansi!(set_top_bot_2, "\x1b[1;43r", [esc(SetTopAndBottom(1, 43))]);
test_parse_ansi!(set_top_bot_3, "\x1b[1;r", [esc(SetTopAndBottom(1, 1))]);
test_parse_ansi!(set_top_bot_4, "\x1b[;1r", [esc(SetTopAndBottom(1, 1))]);
test_parse_ansi!(set_top_bot_5, "\x1b[;r", [esc(SetTopAndBottom(1, 1))]);
test_parse_ansi!(
    set_top_bot_6,
    "\x1b[500;500r",
    [esc(SetTopAndBottom(500, 500))]
);
test_parse_ansi!(cur_save, "\x1b[s", [esc(CursorSave)]);
test_parse_ansi!(cur_res, "\x1b[u", [esc(CursorRestore)]);
test_parse_ansi!(erase_dis, "\x1b[2J", [esc(EraseDisplay)]);
test_parse_ansi!(erase_line, "\x1b[K", [esc(EraseLine)]);
test_parse_ansi!(cur_hide, "\x1b[?25l", [esc(HideCursor)]);
test_parse_ansi!(cur_show, "\x1b[?25h", [esc(ShowCursor)]);
test_parse_ansi!(cur_to_app, "\x1b[?1h", [esc(CursorToApp)]);
test_parse_ansi!(set_n_line_mode, "\x1b[20h", [esc(SetNewLineMode)]);
test_parse_ansi!(set_col132, "\x1b[?3h", [esc(SetCol132)]);
test_parse_ansi!(set_smoot_scroll, "\x1b[?4h", [esc(SetSmoothScroll)]);
test_parse_ansi!(set_reverse_video, "\x1b[?5h", [esc(SetReverseVideo)]);
test_parse_ansi!(set_origin_relative, "\x1b[?6h", [esc(SetOriginRelative)]);
test_parse_ansi!(set_auto_wrap, "\x1b[?7h", [esc(SetAutoWrap)]);
test_parse_ansi!(set_auto_repeat, "\x1b[?8h", [esc(SetAutoRepeat)]);
test_parse_ansi!(set_interlacing, "\x1b[?9h", [esc(SetInterlacing)]);
test_parse_ansi!(set_line_feed_mode, "\x1b[20l", [esc(SetLineFeedMode)]);
test_parse_ansi!(set_cur_key_cur, "\x1b[?1l", [esc(SetCursorKeyToCursor)]);
test_parse_ansi!(set_vt52, "\x1b[?2l", [esc(SetVT52)]);
test_parse_ansi!(set_col80, "\x1b[?3l", [esc(SetCol80)]);
test_parse_ansi!(set_jump_scroll, "\x1b[?4l", [esc(SetJumpScrolling)]);
test_parse_ansi!(set_norm_video, "\x1b[?5l", [esc(SetNormalVideo)]);
test_parse_ansi!(set_origin_abs, "\x1b[?6l", [esc(SetOriginAbsolute)]);
test_parse_ansi!(reset_autowrap, "\x1b[?7l", [esc(ResetAutoWrap)]);
test_parse_ansi!(reset_autorepeat, "\x1b[?8l", [esc(ResetAutoRepeat)]);
test_parse_ansi!(reset_interlacing, "\x1b[?9l", [esc(ResetInterlacing)]);
test_parse_ansi!(set_alt_keypad, "\x1b=", [esc(SetAlternateKeypad)]);
test_parse_ansi!(set_num_keypad, "\x1b>", [esc(SetNumericKeypad)]);
test_parse_ansi!(set_ukg0, "\x1b(A", [esc(SetUKG0)]);
test_parse_ansi!(set_ukg1, "\x1b)A", [esc(SetUKG1)]);
test_parse_ansi!(set_usg0, "\x1b(B", [esc(SetUSG0)]);
test_parse_ansi!(set_usg1, "\x1b)B", [esc(SetUSG1)]);
test_parse_ansi!(set_g0_spec_chars, "\x1b(0", [esc(SetG0SpecialChars)]);
test_parse_ansi!(set_g1_spec_chars, "\x1b)0", [esc(SetG1SpecialChars)]);
test_parse_ansi!(set_g0_alt_chars, "\x1b(1", [esc(SetG0AlternateChar)]);
test_parse_ansi!(set_g1_alt_chars, "\x1b)1", [esc(SetG1AlternateChar)]);
test_parse_ansi!(
    set_g0_spec_alt_chars,
    "\x1b(2",
    [esc(SetG0AltAndSpecialGraph)]
);
test_parse_ansi!(
    set_g1_spec_alt_chars,
    "\x1b)2",
    [esc(SetG1AltAndSpecialGraph)]
);
test_parse_ansi!(set_single_shft2, "\x1bN", [esc(SetSingleShift2)]);
test_parse_ansi!(set_single_shft3, "\x1bO", [esc(SetSingleShift3)]);

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
