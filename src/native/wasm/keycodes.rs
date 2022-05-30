//! Translation of JS key/mouse codes to miniquads'
//! TODO: JS can send better, more easy to use data and most of this file should
//! go away.

use crate::event::{KeyCode, KeyMods, MouseButton, TouchPhase};

pub fn translate_mouse_button(button: i32) -> MouseButton {
    match button {
        0 => return MouseButton::Left,
        1 => return MouseButton::Right,
        2 => return MouseButton::Middle,
        _ => return MouseButton::Unknown,
    };
}
pub fn translate_mod(wasm_mods: i32) -> KeyMods {
    const SAPP_MODIFIER_SHIFT: i32 = 1;
    const SAPP_MODIFIER_CTRL: i32 = 2;
    const SAPP_MODIFIER_ALT: i32 = 4;
    const SAPP_MODIFIER_SUPER: i32 = 8;

    let mut mods = KeyMods::default();
    if wasm_mods & SAPP_MODIFIER_SHIFT != 0 {
        mods.shift = true;
    }
    if wasm_mods & SAPP_MODIFIER_CTRL != 0 {
        mods.ctrl = true;
    }
    if wasm_mods & SAPP_MODIFIER_ALT != 0 {
        mods.alt = true;
    }
    if wasm_mods & SAPP_MODIFIER_SUPER != 0 {
        mods.logo = true;
    }
    return mods;
}

pub fn translate_keycode(keycode: i32) -> KeyCode {
    match keycode {
        32 => KeyCode::Space,
        39 => KeyCode::Apostrophe,
        44 => KeyCode::Comma,
        45 => KeyCode::Minus,
        46 => KeyCode::Period,
        47 => KeyCode::Slash,
        48 => KeyCode::Key0,
        49 => KeyCode::Key1,
        50 => KeyCode::Key2,
        51 => KeyCode::Key3,
        52 => KeyCode::Key4,
        53 => KeyCode::Key5,
        54 => KeyCode::Key6,
        55 => KeyCode::Key7,
        56 => KeyCode::Key8,
        57 => KeyCode::Key9,
        59 => KeyCode::Semicolon,
        61 => KeyCode::Equal,
        65 => KeyCode::A,
        66 => KeyCode::B,
        67 => KeyCode::C,
        68 => KeyCode::D,
        69 => KeyCode::E,
        70 => KeyCode::F,
        71 => KeyCode::G,
        72 => KeyCode::H,
        73 => KeyCode::I,
        74 => KeyCode::J,
        75 => KeyCode::K,
        76 => KeyCode::L,
        77 => KeyCode::M,
        78 => KeyCode::N,
        79 => KeyCode::O,
        80 => KeyCode::P,
        81 => KeyCode::Q,
        82 => KeyCode::R,
        83 => KeyCode::S,
        84 => KeyCode::T,
        85 => KeyCode::U,
        86 => KeyCode::V,
        87 => KeyCode::W,
        88 => KeyCode::X,
        89 => KeyCode::Y,
        90 => KeyCode::Z,
        91 => KeyCode::LeftBracket,
        92 => KeyCode::Backslash,
        93 => KeyCode::RightBracket,
        96 => KeyCode::Apostrophe,
        256 => KeyCode::Escape,
        257 => KeyCode::Enter,
        258 => KeyCode::Tab,
        259 => KeyCode::Backspace,
        260 => KeyCode::Insert,
        261 => KeyCode::Delete,
        262 => KeyCode::Right,
        263 => KeyCode::Left,
        264 => KeyCode::Down,
        265 => KeyCode::Up,
        266 => KeyCode::PageUp,
        267 => KeyCode::PageDown,
        268 => KeyCode::Home,
        269 => KeyCode::End,
        280 => KeyCode::CapsLock,
        281 => KeyCode::ScrollLock,
        282 => KeyCode::NumLock,
        283 => KeyCode::PrintScreen,
        284 => KeyCode::Pause,
        290 => KeyCode::F1,
        291 => KeyCode::F2,
        292 => KeyCode::F3,
        293 => KeyCode::F4,
        294 => KeyCode::F5,
        295 => KeyCode::F6,
        296 => KeyCode::F7,
        297 => KeyCode::F8,
        298 => KeyCode::F9,
        299 => KeyCode::F10,
        300 => KeyCode::F11,
        301 => KeyCode::F12,
        302 => KeyCode::F13,
        303 => KeyCode::F14,
        304 => KeyCode::F15,
        305 => KeyCode::F16,
        306 => KeyCode::F17,
        307 => KeyCode::F18,
        308 => KeyCode::F19,
        309 => KeyCode::F20,
        310 => KeyCode::F21,
        311 => KeyCode::F22,
        312 => KeyCode::F23,
        313 => KeyCode::F24,
        320 => KeyCode::Kp0,
        321 => KeyCode::Kp1,
        322 => KeyCode::Kp2,
        323 => KeyCode::Kp3,
        324 => KeyCode::Kp4,
        325 => KeyCode::Kp5,
        326 => KeyCode::Kp6,
        327 => KeyCode::Kp7,
        328 => KeyCode::Kp8,
        329 => KeyCode::Kp9,
        330 => KeyCode::KpDecimal,
        331 => KeyCode::KpDivide,
        332 => KeyCode::KpMultiply,
        333 => KeyCode::KpSubtract,
        334 => KeyCode::KpAdd,
        335 => KeyCode::KpEnter,
        336 => KeyCode::KpEqual,
        340 => KeyCode::LeftShift,
        341 => KeyCode::LeftControl,
        342 => KeyCode::LeftAlt,
        343 => KeyCode::LeftSuper,
        344 => KeyCode::RightShift,
        345 => KeyCode::RightControl,
        346 => KeyCode::RightAlt,
        347 => KeyCode::RightSuper,
        348 => KeyCode::Menu,
        _ => KeyCode::Unknown,
    }
}

pub fn translate_touch_phase(phase: i32) -> TouchPhase {
    const TOUCHES_BEGAN: i32 = 10;
    const TOUCHES_MOVED: i32 = 11;
    const TOUCHES_ENDED: i32 = 12;
    const TOUCHES_CANCELLED: i32 = 13;

    match phase {
        TOUCHES_BEGAN => TouchPhase::Started,
        TOUCHES_MOVED => TouchPhase::Moved,
        TOUCHES_ENDED => TouchPhase::Ended,
        TOUCHES_CANCELLED => TouchPhase::Cancelled,
        _ => TouchPhase::Moved,
    }
}
