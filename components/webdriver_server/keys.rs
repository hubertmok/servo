/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use msg::constellation_msg::{Key, KeyState, KeyModifiers};

/// Takes a character and returns an Option containing a tuple of the
/// corresponding keycode and whether shift is required. This is
/// currently pretty much ascii-only and the webdriver spec isn't
/// entirely clear on how to deal with characters outside this
/// range. Returns None if no key corresponding to the character is
/// matched.
fn key_from_char(key_string: &char) -> Option<(Key, bool)> {
    match *key_string {
        ' ' => Some((Key::Space, false)),
        '\'' => Some((Key::Apostrophe, true)),
        '\"' => Some((Key::Apostrophe, false)),
        '<' => Some((Key::Comma, true)),
        ',' => Some((Key::Comma, false)),
        '_' => Some((Key::Minus, true)),
        '-' => Some((Key::Minus, false)),
        '>' => Some((Key::Period, true)),
        '.' => Some((Key::Period, false)),
        '?' => Some((Key::Slash, true)),
        '/' => Some((Key::Slash, false)),
        '~' => Some((Key::GraveAccent, true)),
        '`' => Some((Key::GraveAccent, false)),
        ')' => Some((Key::Num0, true)),
        '0' => Some((Key::Num0, false)),
        '!' => Some((Key::Num1, true)),
        '1' => Some((Key::Num1, false)),
        '@' => Some((Key::Num2, true)),
        '2' => Some((Key::Num2, false)),
        '#' => Some((Key::Num3, true)),
        '3' => Some((Key::Num3, false)),
        '$' => Some((Key::Num4, true)),
        '4' => Some((Key::Num4, false)),
        '%' => Some((Key::Num5, true)),
        '5' => Some((Key::Num5, false)),
        '^' => Some((Key::Num6, true)),
        '6' => Some((Key::Num6, false)),
        '&' => Some((Key::Num7, true)),
        '7' => Some((Key::Num7, false)),
        '*' => Some((Key::Num8, true)),
        '8' => Some((Key::Num8, false)),
        '(' => Some((Key::Num9, true)),
        '9' => Some((Key::Num9, false)),
        ':' => Some((Key::Semicolon, true)),
        ';' => Some((Key::Semicolon, false)),
        '+' => Some((Key::Equal, true)),
        '=' => Some((Key::Equal, false)),
        'A' => Some((Key::A, true)),
        'a' => Some((Key::A, false)),
        'B' => Some((Key::B, true)),
        'b' => Some((Key::B, false)),
        'C' => Some((Key::C, true)),
        'c' => Some((Key::C, false)),
        'D' => Some((Key::D, true)),
        'd' => Some((Key::D, false)),
        'E' => Some((Key::E, true)),
        'e' => Some((Key::E, false)),
        'F' => Some((Key::F, true)),
        'f' => Some((Key::F, false)),
        'G' => Some((Key::G, true)),
        'g' => Some((Key::G, false)),
        'H' => Some((Key::H, true)),
        'h' => Some((Key::H, false)),
        'I' => Some((Key::I, true)),
        'i' => Some((Key::I, false)),
        'J' => Some((Key::J, true)),
        'j' => Some((Key::J, false)),
        'K' => Some((Key::K, true)),
        'k' => Some((Key::K, false)),
        'L' => Some((Key::L, true)),
        'l' => Some((Key::L, false)),
        'M' => Some((Key::M, true)),
        'm' => Some((Key::M, false)),
        'N' => Some((Key::N, true)),
        'n' => Some((Key::N, false)),
        'O' => Some((Key::O, true)),
        'o' => Some((Key::O, false)),
        'P' => Some((Key::P, true)),
        'p' => Some((Key::P, false)),
        'Q' => Some((Key::Q, true)),
        'q' => Some((Key::Q, false)),
        'R' => Some((Key::R, true)),
        'r' => Some((Key::R, false)),
        'S' => Some((Key::S, true)),
        's' => Some((Key::S, false)),
        'T' => Some((Key::T, true)),
        't' => Some((Key::T, false)),
        'U' => Some((Key::U, true)),
        'u' => Some((Key::U, false)),
        'V' => Some((Key::V, true)),
        'v' => Some((Key::V, false)),
        'W' => Some((Key::W, true)),
        'w' => Some((Key::W, false)),
        'X' => Some((Key::X, true)),
        'x' => Some((Key::X, false)),
        'Y' => Some((Key::Y, true)),
        'y' => Some((Key::Y, false)),
        'Z' => Some((Key::Z, true)),
        'z' => Some((Key::Z, false)),
        '{' => Some((Key::LeftBracket, true)),
        '[' => Some((Key::LeftBracket, false)),
        '|' => Some((Key::Backslash, true)),
        '\\' => Some((Key::Backslash, false)),
        '}' => Some((Key::RightBracket, true)),
        ']' => Some((Key::RightBracket, false)),
        '\u{E000}' => None,
        '\u{E001}' => None,
        '\u{E002}' => None,
        '\u{E003}' => Some((Key::Backspace, false)),
        '\u{E004}' => Some((Key::Tab, false)),
        '\u{E005}' => None,
        '\u{E006}' => Some((Key::Enter, false)), // This is supposed to be the Return key
        '\u{E007}' => Some((Key::Enter, false)),
        '\u{E008}' => Some((Key::LeftShift, false)),
        '\u{E009}' => Some((Key::LeftShift, false)),
        '\u{E00A}' => Some((Key::LeftAlt, false)),
        '\u{E00B}' => Some((Key::Pause, false)),
        '\u{E00C}' => Some((Key::Escape, false)),
        '\u{E00D}' => Some((Key::Space, false)),
        '\u{E00E}' => Some((Key::PageUp, false)),
        '\u{E00F}' => Some((Key::PageDown, false)),
        '\u{E010}' => Some((Key::End, false)),
        '\u{E011}' => Some((Key::Home, false)),
        '\u{E012}' => Some((Key::Right, false)),
        '\u{E013}' => Some((Key::Left, false)),
        '\u{E014}' => Some((Key::Down, false)),
        '\u{E015}' => Some((Key::Up, false)),
        '\u{E016}' => Some((Key::Insert, false)),
        '\u{E017}' => Some((Key::Delete, false)),
        '\u{E018}' => Some((Key::Semicolon, false)),
        '\u{E019}' => Some((Key::Equal, false)),
        '\u{E01A}' => Some((Key::Kp0, false)),
        '\u{E01B}' => Some((Key::Kp1, false)),
        '\u{E01C}' => Some((Key::Kp2, false)),
        '\u{E01D}' => Some((Key::Kp3, false)),
        '\u{E01E}' => Some((Key::Kp4, false)),
        '\u{E01F}' => Some((Key::Kp5, false)),
        '\u{E020}' => Some((Key::Kp6, false)),
        '\u{E021}' => Some((Key::Kp7, false)),
        '\u{E022}' => Some((Key::Kp8, false)),
        '\u{E023}' => Some((Key::Kp9, false)),
        '\u{E024}' => Some((Key::KpMultiply, false)),
        '\u{E025}' => Some((Key::KpAdd, false)),
        '\u{E026}' => Some((Key::KpEnter, false)),
        '\u{E027}' => Some((Key::KpSubtract, false)),
        '\u{E028}' => Some((Key::KpDecimal, false)),
        '\u{E029}' => Some((Key::KpDivide, false)),
        '\u{E031}' => Some((Key::F1, false)),
        '\u{E032}' => Some((Key::F2, false)),
        '\u{E033}' => Some((Key::F3, false)),
        '\u{E034}' => Some((Key::F4, false)),
        '\u{E035}' => Some((Key::F5, false)),
        '\u{E036}' => Some((Key::F6, false)),
        '\u{E037}' => Some((Key::F7, false)),
        '\u{E038}' => Some((Key::F8, false)),
        '\u{E039}' => Some((Key::F9, false)),
        '\u{E03A}' => Some((Key::F10, false)),
        '\u{E03B}' => Some((Key::F11, false)),
        '\u{E03C}' => Some((Key::F12, false)),
        '\u{E03D}' => None,
        '\u{E040}' => None,
        _ => None,
    }
}

pub fn keycodes_to_keys(key_codes: &str) -> Result<Vec<(Key, KeyModifiers, KeyState)>, String> {
    let mut rv = vec![];

    for char_code in key_codes.chars() {
        let (key, with_shift) =
            key_from_char(&char_code).ok_or(format!("Unsupported character code {}", char_code))?;
        let modifiers = if with_shift {
            KeyModifiers::SHIFT
        } else {
            KeyModifiers::empty()
        };
        rv.push((key, modifiers, KeyState::Pressed));
        rv.push((key, modifiers, KeyState::Released));
    }
    Ok(rv)
}
