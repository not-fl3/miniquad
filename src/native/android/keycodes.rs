use crate::event::KeyCode;

/// Translate an Android `AKEYCODE_*` value into miniquad's
/// [`KeyCode`]. Keep entries ordered by numeric value; the comments
/// reference the `KEYCODE_*` symbol from Android's `KeyEvent.java`
/// so each line is self-explanatory at a glance.
pub fn translate_keycode(keycode: u32) -> KeyCode {
    match keycode {
        0x01 => KeyCode::Left,         // KEYCODE_SOFT_LEFT
        0x02 => KeyCode::Right,        // KEYCODE_SOFT_RIGHT
        0x03 => KeyCode::Home,         // KEYCODE_HOME (device home button)
        0x04 => KeyCode::Back,         // KEYCODE_BACK
        0x07 => KeyCode::Key0,         // KEYCODE_0
        0x08 => KeyCode::Key1,         // KEYCODE_1
        0x09 => KeyCode::Key2,         // KEYCODE_2
        0x0a => KeyCode::Key3,         // KEYCODE_3
        0x0b => KeyCode::Key4,         // KEYCODE_4
        0x0c => KeyCode::Key5,         // KEYCODE_5
        0x0d => KeyCode::Key6,         // KEYCODE_6
        0x0e => KeyCode::Key7,         // KEYCODE_7
        0x0f => KeyCode::Key8,         // KEYCODE_8
        0x10 => KeyCode::Key9,         // KEYCODE_9
        0x13 => KeyCode::Up,           // KEYCODE_DPAD_UP
        0x14 => KeyCode::Down,         // KEYCODE_DPAD_DOWN
        0x15 => KeyCode::Left,         // KEYCODE_DPAD_LEFT
        0x16 => KeyCode::Right,        // KEYCODE_DPAD_RIGHT
        0x17 => KeyCode::Enter,        // KEYCODE_DPAD_CENTER
        0x1d => KeyCode::A,            // KEYCODE_A
        0x1e => KeyCode::B,
        0x1f => KeyCode::C,
        0x20 => KeyCode::D,
        0x21 => KeyCode::E,
        0x22 => KeyCode::F,
        0x23 => KeyCode::G,
        0x24 => KeyCode::H,
        0x25 => KeyCode::I,
        0x26 => KeyCode::J,
        0x27 => KeyCode::K,
        0x28 => KeyCode::L,
        0x29 => KeyCode::M,
        0x2a => KeyCode::N,
        0x2b => KeyCode::O,
        0x2c => KeyCode::P,
        0x2d => KeyCode::Q,
        0x2e => KeyCode::R,
        0x2f => KeyCode::S,
        0x30 => KeyCode::T,
        0x31 => KeyCode::U,
        0x32 => KeyCode::V,
        0x33 => KeyCode::W,
        0x34 => KeyCode::X,
        0x35 => KeyCode::Y,
        0x36 => KeyCode::Z,            // KEYCODE_Z
        0x37 => KeyCode::Comma,        // KEYCODE_COMMA
        0x38 => KeyCode::Period,       // KEYCODE_PERIOD
        0x39 => KeyCode::LeftAlt,      // KEYCODE_ALT_LEFT
        0x3a => KeyCode::RightAlt,     // KEYCODE_ALT_RIGHT
        0x3b => KeyCode::LeftShift,    // KEYCODE_SHIFT_LEFT
        0x3c => KeyCode::RightShift,   // KEYCODE_SHIFT_RIGHT
        0x3d => KeyCode::Tab,          // KEYCODE_TAB
        0x3e => KeyCode::Space,        // KEYCODE_SPACE
        0x42 => KeyCode::Enter,        // KEYCODE_ENTER
        // KEYCODE_DEL — Android names it "Delete" but the key has the
        // Backspace icon + behaviour.
        0x43 => KeyCode::Backspace,
        0x44 => KeyCode::GraveAccent,  // KEYCODE_GRAVE
        0x46 => KeyCode::Equal,        // KEYCODE_EQUALS
        0x47 => KeyCode::LeftBracket,  // KEYCODE_LEFT_BRACKET
        0x48 => KeyCode::RightBracket, // KEYCODE_RIGHT_BRACKET
        0x49 => KeyCode::Backslash,    // KEYCODE_BACKSLASH
        0x4a => KeyCode::Semicolon,    // KEYCODE_SEMICOLON
        0x4c => KeyCode::Slash,        // KEYCODE_SLASH
        0x52 => KeyCode::Menu,         // KEYCODE_MENU
        0x5c => KeyCode::PageUp,       // KEYCODE_PAGE_UP
        0x5d => KeyCode::PageDown,     // KEYCODE_PAGE_DOWN
        0x6f => KeyCode::Escape,       // KEYCODE_ESCAPE
        0x70 => KeyCode::Delete,       // KEYCODE_FORWARD_DEL
        0x71 => KeyCode::LeftControl,  // KEYCODE_CTRL_LEFT
        0x72 => KeyCode::RightControl, // KEYCODE_CTRL_RIGHT
        0x73 => KeyCode::CapsLock,     // KEYCODE_CAPS_LOCK
        0x74 => KeyCode::ScrollLock,   // KEYCODE_SCROLL_LOCK
        0x75 => KeyCode::LeftSuper,    // KEYCODE_META_LEFT
        0x76 => KeyCode::RightSuper,   // KEYCODE_META_RIGHT
        0x78 => KeyCode::PrintScreen,  // KEYCODE_SYSRQ
        0x79 => KeyCode::Pause,        // KEYCODE_BREAK
        0x7a => KeyCode::Home,         // KEYCODE_MOVE_HOME (cursor home)
        0x7b => KeyCode::End,          // KEYCODE_MOVE_END
        0x7c => KeyCode::Insert,       // KEYCODE_INSERT
        0x83 => KeyCode::F1,           // KEYCODE_F1
        0x84 => KeyCode::F2,
        0x85 => KeyCode::F3,
        0x86 => KeyCode::F4,
        0x87 => KeyCode::F5,
        0x88 => KeyCode::F6,
        0x89 => KeyCode::F7,
        0x8a => KeyCode::F8,
        0x8b => KeyCode::F9,
        0x8c => KeyCode::F10,
        0x8d => KeyCode::F11,
        0x8e => KeyCode::F12,          // KEYCODE_F12
        0x8f => KeyCode::NumLock,      // KEYCODE_NUM_LOCK
        0x90 => KeyCode::Kp0,          // KEYCODE_NUMPAD_0
        0x91 => KeyCode::Kp1,
        0x92 => KeyCode::Kp2,
        0x93 => KeyCode::Kp3,
        0x94 => KeyCode::Kp4,
        0x95 => KeyCode::Kp5,
        0x96 => KeyCode::Kp6,
        0x97 => KeyCode::Kp7,
        0x98 => KeyCode::Kp8,
        0x99 => KeyCode::Kp9,          // KEYCODE_NUMPAD_9
        0x9a => KeyCode::KpDivide,     // KEYCODE_NUMPAD_DIVIDE
        0x9b => KeyCode::KpMultiply,   // KEYCODE_NUMPAD_MULTIPLY
        0x9c => KeyCode::KpSubtract,   // KEYCODE_NUMPAD_SUBTRACT
        0x9d => KeyCode::KpAdd,        // KEYCODE_NUMPAD_ADD
        0x9e => KeyCode::KpDecimal,    // KEYCODE_NUMPAD_DOT
        0xa0 => KeyCode::KpEnter,      // KEYCODE_NUMPAD_ENTER
        0xa1 => KeyCode::KpEqual,      // KEYCODE_NUMPAD_EQUALS
        _ => KeyCode::Unknown,
    }
}
