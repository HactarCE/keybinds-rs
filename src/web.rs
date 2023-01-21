//! Web-specific functionality.

use super::*;

#[cfg(feature = "winit")]
pub use crate::common::key_map_to_winit_vkey;

pub(crate) const ALT_STR: &str = "Alt";
pub(crate) const LOGO_STR: &str = "Logo";
pub(crate) const MODIFIERS_ORDER: &str = "csam"; // Ctrl + Shift + Alt + Meta

pub(crate) const SC_INVALID: u16 = 0x0000;
pub(crate) const SC_TO_KEY_MAPPING: fn(u16) -> KeyMapping = arbitrary_scancode_to_key_mapping;
pub(crate) const KEY_MAP_TO_SC: fn(KeyMap) -> u16 = key_map_to_arbitrary_scancode;

pub(crate) fn scancode_name(sc: u16) -> String {
    match arbitrary_scancode_to_winit_vkey(sc) {
        Some(key_code) => format!("{:?}", key_code),
        None => format!("SC{}", sc),
    }
}

/// Undoes winit's switching of keycode and scancode by returning a fake
/// scancode (used only by this crate) from a virtual key code.
pub fn winit_vkey_to_arbitrary_scancode(keycode: winit::event::VirtualKeyCode) -> u16 {
    use winit::event::VirtualKeyCode::*;

    // Use made-up scancodes.
    match keycode {
        Key1 => 1,
        Key2 => 2,
        Key3 => 3,
        Key4 => 4,
        Key5 => 5,
        Key6 => 6,
        Key7 => 7,
        Key8 => 8,
        Key9 => 9,
        Key0 => 10,
        A => 11,
        B => 12,
        C => 13,
        D => 14,
        E => 15,
        F => 16,
        G => 17,
        H => 18,
        I => 19,
        J => 20,
        K => 21,
        L => 22,
        M => 23,
        N => 24,
        O => 25,
        P => 26,
        Q => 27,
        R => 28,
        S => 29,
        T => 30,
        U => 31,
        V => 32,
        W => 33,
        X => 34,
        Y => 35,
        Z => 36,
        Escape => 37,
        F1 => 38,
        F2 => 39,
        F3 => 40,
        F4 => 41,
        F5 => 42,
        F6 => 43,
        F7 => 44,
        F8 => 45,
        F9 => 46,
        F10 => 47,
        F11 => 48,
        F12 => 49,
        F13 => 50,
        F14 => 51,
        F15 => 52,
        F16 => 53,
        F17 => 54,
        F18 => 55,
        F19 => 56,
        F20 => 57,
        F21 => 58,
        F22 => 59,
        F23 => 60,
        F24 => 61,
        Snapshot => 62,
        Scroll => 63,
        Pause => 64,
        Insert => 65,
        Home => 66,
        Delete => 67,
        End => 68,
        PageDown => 69,
        PageUp => 70,
        Left => 71,
        Up => 72,
        Right => 73,
        Down => 74,
        Back => 75,
        Return => 76,
        Space => 77,
        Compose => 78,
        Caret => 79,
        Numlock => 80,
        Numpad0 => 81,
        Numpad1 => 82,
        Numpad2 => 83,
        Numpad3 => 84,
        Numpad4 => 85,
        Numpad5 => 86,
        Numpad6 => 87,
        Numpad7 => 88,
        Numpad8 => 89,
        Numpad9 => 90,
        NumpadAdd => 91,
        NumpadDivide => 92,
        NumpadDecimal => 93,
        NumpadComma => 94,
        NumpadEnter => 95,
        NumpadEquals => 96,
        NumpadMultiply => 97,
        NumpadSubtract => 98,
        AbntC1 => 99,
        AbntC2 => 100,
        Apostrophe => 101,
        Apps => 102,
        Asterisk => 103,
        At => 104,
        Ax => 105,
        Backslash => 106,
        Calculator => 107,
        Capital => 108,
        Colon => 109,
        Comma => 110,
        Convert => 111,
        Equals => 112,
        Grave => 113,
        Kana => 114,
        Kanji => 115,
        LAlt => 116,
        LBracket => 117,
        LControl => 118,
        LShift => 119,
        LWin => 120,
        Mail => 121,
        MediaSelect => 122,
        MediaStop => 123,
        Minus => 124,
        Mute => 125,
        MyComputer => 126,
        NavigateForward => 127,
        NavigateBackward => 128,
        NextTrack => 129,
        NoConvert => 130,
        OEM102 => 131,
        Period => 132,
        PlayPause => 133,
        Plus => 134,
        Power => 135,
        PrevTrack => 136,
        RAlt => 137,
        RBracket => 138,
        RControl => 139,
        RShift => 140,
        RWin => 141,
        Semicolon => 142,
        Slash => 143,
        Sleep => 144,
        Stop => 145,
        Sysrq => 146,
        Tab => 147,
        Underline => 148,
        Unlabeled => 149,
        VolumeDown => 150,
        VolumeUp => 151,
        Wake => 152,
        WebBack => 153,
        WebFavorites => 154,
        WebForward => 155,
        WebHome => 156,
        WebRefresh => 157,
        WebSearch => 158,
        WebStop => 159,
        Yen => 160,
        Copy => 161,
        Paste => 162,
        Cut => 163,
    }
}

/// Returns the virtual key code from the fake scancodes used by this crate.
pub fn arbitrary_scancode_to_winit_vkey(scancode: u16) -> Option<winit::event::VirtualKeyCode> {
    use winit::event::VirtualKeyCode::*;

    // Use made-up scancodes.
    Some(match scancode {
        1 => Key1,
        2 => Key2,
        3 => Key3,
        4 => Key4,
        5 => Key5,
        6 => Key6,
        7 => Key7,
        8 => Key8,
        9 => Key9,
        10 => Key0,
        11 => A,
        12 => B,
        13 => C,
        14 => D,
        15 => E,
        16 => F,
        17 => G,
        18 => H,
        19 => I,
        20 => J,
        21 => K,
        22 => L,
        23 => M,
        24 => N,
        25 => O,
        26 => P,
        27 => Q,
        28 => R,
        29 => S,
        30 => T,
        31 => U,
        32 => V,
        33 => W,
        34 => X,
        35 => Y,
        36 => Z,
        37 => Escape,
        38 => F1,
        39 => F2,
        40 => F3,
        41 => F4,
        42 => F5,
        43 => F6,
        44 => F7,
        45 => F8,
        46 => F9,
        47 => F10,
        48 => F11,
        49 => F12,
        50 => F13,
        51 => F14,
        52 => F15,
        53 => F16,
        54 => F17,
        55 => F18,
        56 => F19,
        57 => F20,
        58 => F21,
        59 => F22,
        60 => F23,
        61 => F24,
        62 => Snapshot,
        63 => Scroll,
        64 => Pause,
        65 => Insert,
        66 => Home,
        67 => Delete,
        68 => End,
        69 => PageDown,
        70 => PageUp,
        71 => Left,
        72 => Up,
        73 => Right,
        74 => Down,
        75 => Back,
        76 => Return,
        77 => Space,
        78 => Compose,
        79 => Caret,
        80 => Numlock,
        81 => Numpad0,
        82 => Numpad1,
        83 => Numpad2,
        84 => Numpad3,
        85 => Numpad4,
        86 => Numpad5,
        87 => Numpad6,
        88 => Numpad7,
        89 => Numpad8,
        90 => Numpad9,
        91 => NumpadAdd,
        92 => NumpadDivide,
        93 => NumpadDecimal,
        94 => NumpadComma,
        95 => NumpadEnter,
        96 => NumpadEquals,
        97 => NumpadMultiply,
        98 => NumpadSubtract,
        99 => AbntC1,
        100 => AbntC2,
        101 => Apostrophe,
        102 => Apps,
        103 => Asterisk,
        104 => At,
        105 => Ax,
        106 => Backslash,
        107 => Calculator,
        108 => Capital,
        109 => Colon,
        110 => Comma,
        111 => Convert,
        112 => Equals,
        113 => Grave,
        114 => Kana,
        115 => Kanji,
        116 => LAlt,
        117 => LBracket,
        118 => LControl,
        119 => LShift,
        120 => LWin,
        121 => Mail,
        122 => MediaSelect,
        123 => MediaStop,
        124 => Minus,
        125 => Mute,
        126 => MyComputer,
        127 => NavigateForward,
        128 => NavigateBackward,
        129 => NextTrack,
        130 => NoConvert,
        131 => OEM102,
        132 => Period,
        133 => PlayPause,
        134 => Plus,
        135 => Power,
        136 => PrevTrack,
        137 => RAlt,
        138 => RBracket,
        139 => RControl,
        140 => RShift,
        141 => RWin,
        142 => Semicolon,
        143 => Slash,
        144 => Sleep,
        145 => Stop,
        146 => Sysrq,
        147 => Tab,
        148 => Underline,
        149 => Unlabeled,
        150 => VolumeDown,
        151 => VolumeUp,
        152 => Wake,
        153 => WebBack,
        154 => WebFavorites,
        155 => WebForward,
        156 => WebHome,
        157 => WebRefresh,
        158 => WebSearch,
        159 => WebStop,
        160 => Yen,
        161 => Copy,
        162 => Paste,
        163 => Cut,
        _ => return None,
    })
}

/// Undoes winit's switching of keycode and scancode by returning a best guess
/// at a virtual key code from an ASCII code given by the browser.
pub fn ascii_to_keycode(ascii: u8) -> Option<winit::event::VirtualKeyCode> {
    use winit::event::VirtualKeyCode::*;

    // from https://www.toptal.com/developers/keycode/table-of-all-keycodes
    //
    // winit uses the deprecated `keyCode` attribute, which isn't standardized
    // (https://www.w3.org/TR/uievents/#dom-keyboardevent-keycode)
    //
    //
    Some(match ascii {
        /* 0 => None, */
        3 => Pause, // also 19
        8 => Back,
        9 => Tab,
        12 => Numlock,
        13 => Return,
        16 => LShift,
        17 => LControl,
        18 => LAlt,
        19 => Pause, // also 3
        20 => Capital,
        /* 21 => Lang1, */
        /* 25 => Lang2, */
        27 => Escape,
        28 => Convert,
        29 => NoConvert,
        32 => Space,
        33 => Numpad9,
        34 => Numpad3,
        35 => Numpad1,
        36 => Numpad7,
        37 => Left,
        38 => Up,
        39 => Right,
        40 => Down,
        /* 41 => select */
        /* 42 => print */
        /* 43 => execute */
        44 => F13,
        45 => Numpad0,
        46 => NumpadDecimal,
        /* 47 => help, */
        48 => Key0,
        49 => Key1,
        50 => Key2,
        51 => Key3,
        52 => Key4,
        53 => Key5,
        54 => Key6,
        55 => Key7,
        56 => Key8,
        57 => Key9,
        58 => Period,
        59 => Semicolon,
        60 => Grave,
        61 => Equals,
        63 => Minus,
        65 => A,
        66 => B,
        67 => C,
        68 => D,
        69 => E,
        70 => F,
        71 => G,
        72 => H,
        73 => I,
        74 => J,
        75 => K,
        76 => L,
        77 => M,
        78 => N,
        79 => O,
        80 => P,
        81 => Q,
        82 => R,
        83 => S,
        84 => T,
        85 => U,
        86 => V,
        87 => W,
        88 => X,
        89 => Y,
        90 => Z,
        91 => LWin,
        92 => RWin,
        93 => Apps,
        95 => Sleep,
        96 => Numpad0,
        97 => Numpad1,
        98 => Numpad2,
        99 => Numpad3,
        100 => Numpad4,
        101 => Numpad5,
        102 => Numpad6,
        103 => Numpad7,
        104 => Numpad8,
        105 => Numpad9,
        106 => NumpadMultiply,
        107 => NumpadAdd,
        108 => NumpadDecimal,
        109 => NumpadSubtract,
        110 => NumpadDecimal,
        111 => NumpadDivide,
        112 => F1,
        113 => F2,
        114 => F3,
        115 => F4,
        116 => F5,
        117 => F6,
        118 => F7,
        119 => F8,
        120 => F9,
        121 => F10,
        122 => F11,
        123 => F12,
        124 => F13,
        125 => F14,
        126 => F15,
        127 => F16,
        128 => F17,
        129 => F18,
        130 => F19,
        131 => F20,
        132 => F21,
        133 => F22,
        134 => F23,
        135 => F24,
        /* 136 => F25, */
        /* 137 => F26, */
        /* 138 => F27, */
        /* 139 => F28, */
        /* 140 => F29, */
        /* 141 => F30, */
        /* 142 => F31, */
        /* 143 => F32, */
        144 => Numlock,
        145 => Scroll,
        160 => LBracket,
        161 => RBracket,
        163 => Grave,
        164 => Backslash,
        169 => Minus,
        170 => Backslash,
        /* 171 => wtf? */
        172 => Home,
        173 => Minus,
        174 => VolumeDown,
        175 => VolumeUp,
        176 => NextTrack,
        177 => PrevTrack,
        178 => Stop,
        179 => PlayPause,
        180 => Mail,
        181 => Mute,
        182 => VolumeDown,
        183 => VolumeUp,
        186 => Semicolon,
        187 => Equals,
        188 => Comma,
        189 => Minus,
        190 => Period,
        191 => Slash,
        192 => Grave,
        /* 193 => IntlRo, */
        194 => NumpadComma,
        219 => LBracket,
        220 => Backslash,
        221 => RBracket,
        222 => Apostrophe,
        223 => Grave,
        224 => LWin,
        225 => RAlt,
        226 => OEM102,
        /* 230 => GNOME Compose Key, */
        /* 231 => ç, */
        233 => NavigateBackward,
        234 => NavigateForward,
        235 => NoConvert,
        242 => Kana,
        /* 243 => hiragana/katakana, */
        244 => Kanji,
        /* 251 => unlock track pad, */
        /* 255 => WakeUp, */
        _ => return None,
    })
}

fn arbitrary_scancode_to_key_mapping(sc: u16) -> KeyMapping {
    match arbitrary_scancode_to_winit_vkey(sc) {
        Some(vkey) => winit_vkey_to_key_mapping(vkey),
        None => KeyMapping::Code(None),
    }
}

/// Converts a virtual key code
fn winit_vkey_to_key_mapping(key: winit::event::VirtualKeyCode) -> KeyMapping {
    use winit::event::VirtualKeyCode as Vk;
    use KeyMappingCode as Km;
    let key_mapping_code = match key {
        Vk::Key0 => Km::Digit0,
        Vk::Key1 => Km::Digit1,
        Vk::Key2 => Km::Digit2,
        Vk::Key3 => Km::Digit3,
        Vk::Key4 => Km::Digit4,
        Vk::Key5 => Km::Digit5,
        Vk::Key6 => Km::Digit6,
        Vk::Key7 => Km::Digit7,
        Vk::Key8 => Km::Digit8,
        Vk::Key9 => Km::Digit9,

        Vk::A => Km::KeyA,
        Vk::B => Km::KeyB,
        Vk::C => Km::KeyC,
        Vk::D => Km::KeyD,
        Vk::E => Km::KeyE,
        Vk::F => Km::KeyF,
        Vk::G => Km::KeyG,
        Vk::H => Km::KeyH,
        Vk::I => Km::KeyI,
        Vk::J => Km::KeyJ,
        Vk::K => Km::KeyK,
        Vk::L => Km::KeyL,
        Vk::M => Km::KeyM,
        Vk::N => Km::KeyN,
        Vk::O => Km::KeyO,
        Vk::P => Km::KeyP,
        Vk::Q => Km::KeyQ,
        Vk::R => Km::KeyR,
        Vk::S => Km::KeyS,
        Vk::T => Km::KeyT,
        Vk::U => Km::KeyU,
        Vk::V => Km::KeyV,
        Vk::W => Km::KeyW,
        Vk::X => Km::KeyX,
        Vk::Y => Km::KeyY,
        Vk::Z => Km::KeyZ,

        Vk::Escape => Km::Escape,

        Vk::F1 => Km::F1,
        Vk::F2 => Km::F2,
        Vk::F3 => Km::F3,
        Vk::F4 => Km::F4,
        Vk::F5 => Km::F5,
        Vk::F6 => Km::F6,
        Vk::F7 => Km::F7,
        Vk::F8 => Km::F8,
        Vk::F9 => Km::F9,
        Vk::F10 => Km::F10,
        Vk::F11 => Km::F11,
        Vk::F12 => Km::F12,
        Vk::F13 => Km::F13,
        Vk::F14 => Km::F14,
        Vk::F15 => Km::F15,
        Vk::F16 => Km::F16,
        Vk::F17 => Km::F17,
        Vk::F18 => Km::F18,
        Vk::F19 => Km::F19,
        Vk::F20 => Km::F20,
        Vk::F21 => Km::F21,
        Vk::F22 => Km::F22,
        Vk::F23 => Km::F23,
        Vk::F24 => Km::F24,

        // Vk::Snapshot => , // `PrintScreen` maps to `Sysrq`
        Vk::Scroll => Km::ScrollLock,
        Vk::Pause => Km::Pause,

        Vk::Insert => Km::Insert,
        Vk::Home => Km::Home,
        Vk::Delete => Km::Delete,
        Vk::End => Km::End,
        Vk::PageDown => Km::PageDown,
        Vk::PageUp => Km::PageUp,

        Vk::Left => Km::ArrowLeft,
        Vk::Up => Km::ArrowUp,
        Vk::Right => Km::ArrowRight,
        Vk::Down => Km::ArrowDown,

        Vk::Back => Km::Backspace,
        Vk::Return => Km::Enter,
        Vk::Space => Km::Space,

        // Vk::Compose => ,

        // Vk::Caret => ,

        //
        Vk::Numlock => Km::NumLock,
        Vk::Numpad0 => Km::Numpad0,
        Vk::Numpad1 => Km::Numpad1,
        Vk::Numpad2 => Km::Numpad2,
        Vk::Numpad3 => Km::Numpad3,
        Vk::Numpad4 => Km::Numpad4,
        Vk::Numpad5 => Km::Numpad5,
        Vk::Numpad6 => Km::Numpad6,
        Vk::Numpad7 => Km::Numpad7,
        Vk::Numpad8 => Km::Numpad8,
        Vk::Numpad9 => Km::Numpad9,
        Vk::NumpadAdd => Km::NumpadAdd,
        Vk::NumpadDivide => Km::NumpadDivide,
        Vk::NumpadDecimal => Km::NumpadDecimal,
        Vk::NumpadComma => Km::NumpadComma,
        Vk::NumpadEnter => Km::NumpadEnter,
        Vk::NumpadEquals => Km::NumpadEqual,
        Vk::NumpadMultiply => Km::NumpadMultiply,
        Vk::NumpadSubtract => Km::NumpadSubtract,

        // Vk::AbntC1 => ,
        // Vk::AbntC2 => ,
        Vk::Apostrophe => Km::Quote,
        Vk::Apps => Km::ContextMenu,
        // Vk::Asterisk => ,
        // Vk::At => ,
        // Vk::Ax => ,
        Vk::Backslash => Km::Backslash,
        Vk::Calculator => Km::LaunchApp2,
        Vk::Capital => Km::CapsLock,
        // Vk::Colon => ,
        Vk::Comma => Km::Comma,
        Vk::Convert => Km::Convert,
        Vk::Equals => Km::Equal,
        Vk::Grave => Km::Backquote,
        Vk::Kana => Km::KanaMode,
        Vk::Kanji => Km::Backquote,
        Vk::LAlt => Km::AltLeft,
        Vk::LBracket => Km::BracketLeft,
        Vk::LControl => Km::ControlLeft,
        Vk::LShift => Km::ShiftLeft,
        Vk::LWin => Km::MetaLeft,
        Vk::Mail => Km::LaunchMail,
        Vk::MediaSelect => Km::MediaSelect,
        Vk::MediaStop => Km::MediaStop,
        Vk::Minus => Km::Minus,
        Vk::Mute => Km::AudioVolumeMute,
        Vk::MyComputer => Km::LaunchApp1,
        Vk::NavigateForward => Km::BrowserForward,
        Vk::NavigateBackward => Km::BrowserForward,
        Vk::NextTrack => Km::MediaTrackNext,
        Vk::NoConvert => Km::NonConvert,
        Vk::OEM102 => Km::IntlBackslash,
        Vk::Period => Km::Period,
        Vk::PlayPause => Km::MediaPlayPause,
        // Vk::Plus => ,
        Vk::Power => Km::Power,
        Vk::PrevTrack => Km::MediaTrackPrevious,
        Vk::RAlt => Km::AltRight,
        Vk::RBracket => Km::BracketRight,
        Vk::RControl => Km::ControlRight,
        Vk::RShift => Km::ShiftRight,
        Vk::RWin => Km::MetaRight,
        Vk::Semicolon => Km::Semicolon,
        Vk::Slash => Km::Slash,
        Vk::Sleep => Km::Sleep,
        // Vk::Stop => ,
        Vk::Sysrq => Km::PrintScreen,
        Vk::Tab => Km::Tab,
        // Vk::Underline => ,
        // Vk::Unlabeled => ,
        Vk::VolumeDown => Km::AudioVolumeDown,
        Vk::VolumeUp => Km::AudioVolumeUp,
        Vk::Wake => Km::WakeUp,
        Vk::WebBack => Km::BrowserBack,
        Vk::WebFavorites => Km::BrowserFavorites,
        Vk::WebForward => Km::BrowserForward,
        Vk::WebHome => Km::BrowserHome,
        Vk::WebRefresh => Km::BrowserRefresh,
        Vk::WebSearch => Km::BrowserSearch,
        Vk::WebStop => Km::BrowserStop,
        Vk::Yen => Km::IntlYen,
        Vk::Copy => Km::Copy,
        Vk::Paste => Km::Paste,
        Vk::Cut => Km::Cut,

        _ => return KeyMapping::Code(None),
    };

    KeyMapping::Code(Some(key_mapping_code))
}

fn key_map_to_arbitrary_scancode(km: KeyMap) -> u16 {
    match key_map_to_winit_vkey(km) {
        Some(vk) => winit_vkey_to_arbitrary_scancode(vk),
        None => SC_INVALID,
    }
}
