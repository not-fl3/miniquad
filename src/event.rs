use crate::Context;

use crate::sapp::{self, sapp_keycode, sapp_mousebutton};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum MouseButton {
    Right,
    Left,
    Middle,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub struct Touch {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

impl From<sapp_mousebutton> for MouseButton {
    fn from(btn: sapp_mousebutton) -> MouseButton {
        match btn {
            sapp::sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT => MouseButton::Left,
            sapp::sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT => MouseButton::Right,
            sapp::sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE => MouseButton::Middle,
            _ => MouseButton::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u32)]
pub enum KeyCode {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}

impl From<sapp_keycode> for KeyCode {
    fn from(key_code: sapp_keycode) -> KeyCode {
        match key_code {
            sapp::sapp_keycode_SAPP_KEYCODE_SPACE => KeyCode::Space,
            sapp::sapp_keycode_SAPP_KEYCODE_APOSTROPHE => KeyCode::Apostrophe,
            sapp::sapp_keycode_SAPP_KEYCODE_COMMA => KeyCode::Comma,
            sapp::sapp_keycode_SAPP_KEYCODE_MINUS => KeyCode::Minus,
            sapp::sapp_keycode_SAPP_KEYCODE_PERIOD => KeyCode::Period,
            sapp::sapp_keycode_SAPP_KEYCODE_SLASH => KeyCode::Slash,
            sapp::sapp_keycode_SAPP_KEYCODE_0 => KeyCode::Key0,
            sapp::sapp_keycode_SAPP_KEYCODE_1 => KeyCode::Key1,
            sapp::sapp_keycode_SAPP_KEYCODE_2 => KeyCode::Key2,
            sapp::sapp_keycode_SAPP_KEYCODE_3 => KeyCode::Key3,
            sapp::sapp_keycode_SAPP_KEYCODE_4 => KeyCode::Key4,
            sapp::sapp_keycode_SAPP_KEYCODE_5 => KeyCode::Key5,
            sapp::sapp_keycode_SAPP_KEYCODE_6 => KeyCode::Key6,
            sapp::sapp_keycode_SAPP_KEYCODE_7 => KeyCode::Key7,
            sapp::sapp_keycode_SAPP_KEYCODE_8 => KeyCode::Key8,
            sapp::sapp_keycode_SAPP_KEYCODE_9 => KeyCode::Key9,
            sapp::sapp_keycode_SAPP_KEYCODE_SEMICOLON => KeyCode::Semicolon,
            sapp::sapp_keycode_SAPP_KEYCODE_EQUAL => KeyCode::Equal,
            sapp::sapp_keycode_SAPP_KEYCODE_A => KeyCode::A,
            sapp::sapp_keycode_SAPP_KEYCODE_B => KeyCode::B,
            sapp::sapp_keycode_SAPP_KEYCODE_C => KeyCode::C,
            sapp::sapp_keycode_SAPP_KEYCODE_D => KeyCode::D,
            sapp::sapp_keycode_SAPP_KEYCODE_E => KeyCode::E,
            sapp::sapp_keycode_SAPP_KEYCODE_F => KeyCode::F,
            sapp::sapp_keycode_SAPP_KEYCODE_G => KeyCode::G,
            sapp::sapp_keycode_SAPP_KEYCODE_H => KeyCode::H,
            sapp::sapp_keycode_SAPP_KEYCODE_I => KeyCode::I,
            sapp::sapp_keycode_SAPP_KEYCODE_J => KeyCode::J,
            sapp::sapp_keycode_SAPP_KEYCODE_K => KeyCode::K,
            sapp::sapp_keycode_SAPP_KEYCODE_L => KeyCode::L,
            sapp::sapp_keycode_SAPP_KEYCODE_M => KeyCode::M,
            sapp::sapp_keycode_SAPP_KEYCODE_N => KeyCode::N,
            sapp::sapp_keycode_SAPP_KEYCODE_O => KeyCode::O,
            sapp::sapp_keycode_SAPP_KEYCODE_P => KeyCode::P,
            sapp::sapp_keycode_SAPP_KEYCODE_Q => KeyCode::Q,
            sapp::sapp_keycode_SAPP_KEYCODE_R => KeyCode::R,
            sapp::sapp_keycode_SAPP_KEYCODE_S => KeyCode::S,
            sapp::sapp_keycode_SAPP_KEYCODE_T => KeyCode::T,
            sapp::sapp_keycode_SAPP_KEYCODE_U => KeyCode::U,
            sapp::sapp_keycode_SAPP_KEYCODE_V => KeyCode::V,
            sapp::sapp_keycode_SAPP_KEYCODE_W => KeyCode::W,
            sapp::sapp_keycode_SAPP_KEYCODE_X => KeyCode::X,
            sapp::sapp_keycode_SAPP_KEYCODE_Y => KeyCode::Y,
            sapp::sapp_keycode_SAPP_KEYCODE_Z => KeyCode::Z,
            sapp::sapp_keycode_SAPP_KEYCODE_LEFT_BRACKET => KeyCode::LeftBracket,
            sapp::sapp_keycode_SAPP_KEYCODE_BACKSLASH => KeyCode::Backslash,
            sapp::sapp_keycode_SAPP_KEYCODE_RIGHT_BRACKET => KeyCode::RightBracket,
            sapp::sapp_keycode_SAPP_KEYCODE_GRAVE_ACCENT => KeyCode::GraveAccent,
            sapp::sapp_keycode_SAPP_KEYCODE_WORLD_1 => KeyCode::World1,
            sapp::sapp_keycode_SAPP_KEYCODE_WORLD_2 => KeyCode::World2,
            sapp::sapp_keycode_SAPP_KEYCODE_ESCAPE => KeyCode::Escape,
            sapp::sapp_keycode_SAPP_KEYCODE_ENTER => KeyCode::Enter,
            sapp::sapp_keycode_SAPP_KEYCODE_TAB => KeyCode::Tab,
            sapp::sapp_keycode_SAPP_KEYCODE_BACKSPACE => KeyCode::Backspace,
            sapp::sapp_keycode_SAPP_KEYCODE_INSERT => KeyCode::Insert,
            sapp::sapp_keycode_SAPP_KEYCODE_DELETE => KeyCode::Delete,
            sapp::sapp_keycode_SAPP_KEYCODE_RIGHT => KeyCode::Right,
            sapp::sapp_keycode_SAPP_KEYCODE_LEFT => KeyCode::Left,
            sapp::sapp_keycode_SAPP_KEYCODE_DOWN => KeyCode::Down,
            sapp::sapp_keycode_SAPP_KEYCODE_UP => KeyCode::Up,
            sapp::sapp_keycode_SAPP_KEYCODE_PAGE_UP => KeyCode::PageUp,
            sapp::sapp_keycode_SAPP_KEYCODE_PAGE_DOWN => KeyCode::PageDown,
            sapp::sapp_keycode_SAPP_KEYCODE_HOME => KeyCode::Home,
            sapp::sapp_keycode_SAPP_KEYCODE_END => KeyCode::End,
            sapp::sapp_keycode_SAPP_KEYCODE_CAPS_LOCK => KeyCode::CapsLock,
            sapp::sapp_keycode_SAPP_KEYCODE_SCROLL_LOCK => KeyCode::ScrollLock,
            sapp::sapp_keycode_SAPP_KEYCODE_NUM_LOCK => KeyCode::NumLock,
            sapp::sapp_keycode_SAPP_KEYCODE_PRINT_SCREEN => KeyCode::PrintScreen,
            sapp::sapp_keycode_SAPP_KEYCODE_PAUSE => KeyCode::Pause,
            sapp::sapp_keycode_SAPP_KEYCODE_F1 => KeyCode::F1,
            sapp::sapp_keycode_SAPP_KEYCODE_F2 => KeyCode::F2,
            sapp::sapp_keycode_SAPP_KEYCODE_F3 => KeyCode::F3,
            sapp::sapp_keycode_SAPP_KEYCODE_F4 => KeyCode::F4,
            sapp::sapp_keycode_SAPP_KEYCODE_F5 => KeyCode::F5,
            sapp::sapp_keycode_SAPP_KEYCODE_F6 => KeyCode::F6,
            sapp::sapp_keycode_SAPP_KEYCODE_F7 => KeyCode::F7,
            sapp::sapp_keycode_SAPP_KEYCODE_F8 => KeyCode::F8,
            sapp::sapp_keycode_SAPP_KEYCODE_F9 => KeyCode::F9,
            sapp::sapp_keycode_SAPP_KEYCODE_F10 => KeyCode::F10,
            sapp::sapp_keycode_SAPP_KEYCODE_F11 => KeyCode::F11,
            sapp::sapp_keycode_SAPP_KEYCODE_F12 => KeyCode::F12,
            sapp::sapp_keycode_SAPP_KEYCODE_F13 => KeyCode::F13,
            sapp::sapp_keycode_SAPP_KEYCODE_F14 => KeyCode::F14,
            sapp::sapp_keycode_SAPP_KEYCODE_F15 => KeyCode::F15,
            sapp::sapp_keycode_SAPP_KEYCODE_F16 => KeyCode::F16,
            sapp::sapp_keycode_SAPP_KEYCODE_F17 => KeyCode::F17,
            sapp::sapp_keycode_SAPP_KEYCODE_F18 => KeyCode::F18,
            sapp::sapp_keycode_SAPP_KEYCODE_F19 => KeyCode::F19,
            sapp::sapp_keycode_SAPP_KEYCODE_F20 => KeyCode::F20,
            sapp::sapp_keycode_SAPP_KEYCODE_F21 => KeyCode::F21,
            sapp::sapp_keycode_SAPP_KEYCODE_F22 => KeyCode::F22,
            sapp::sapp_keycode_SAPP_KEYCODE_F23 => KeyCode::F23,
            sapp::sapp_keycode_SAPP_KEYCODE_F24 => KeyCode::F24,
            sapp::sapp_keycode_SAPP_KEYCODE_F25 => KeyCode::F25,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_0 => KeyCode::Kp0,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_1 => KeyCode::Kp1,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_2 => KeyCode::Kp2,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_3 => KeyCode::Kp3,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_4 => KeyCode::Kp4,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_5 => KeyCode::Kp5,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_6 => KeyCode::Kp6,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_7 => KeyCode::Kp7,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_8 => KeyCode::Kp8,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_9 => KeyCode::Kp9,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_DECIMAL => KeyCode::KpDecimal,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_DIVIDE => KeyCode::KpDivide,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_MULTIPLY => KeyCode::KpMultiply,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_SUBTRACT => KeyCode::KpSubtract,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_ADD => KeyCode::KpAdd,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_ENTER => KeyCode::KpEnter,
            sapp::sapp_keycode_SAPP_KEYCODE_KP_EQUAL => KeyCode::KpEqual,
            sapp::sapp_keycode_SAPP_KEYCODE_LEFT_SHIFT => KeyCode::LeftShift,
            sapp::sapp_keycode_SAPP_KEYCODE_LEFT_CONTROL => KeyCode::LeftControl,
            sapp::sapp_keycode_SAPP_KEYCODE_LEFT_ALT => KeyCode::LeftAlt,
            sapp::sapp_keycode_SAPP_KEYCODE_LEFT_SUPER => KeyCode::LeftSuper,
            sapp::sapp_keycode_SAPP_KEYCODE_RIGHT_SHIFT => KeyCode::RightShift,
            sapp::sapp_keycode_SAPP_KEYCODE_RIGHT_CONTROL => KeyCode::RightControl,
            sapp::sapp_keycode_SAPP_KEYCODE_RIGHT_ALT => KeyCode::RightAlt,
            sapp::sapp_keycode_SAPP_KEYCODE_RIGHT_SUPER => KeyCode::RightSuper,
            sapp::sapp_keycode_SAPP_KEYCODE_MENU => KeyCode::Menu,
            _ => KeyCode::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

impl From<u32> for KeyMods {
    fn from(value: u32) -> KeyMods {
        let mut key_mods = KeyMods::default();

        if value & sapp::SAPP_MODIFIER_SHIFT != 0 {
            key_mods.shift = true;
        }
        if value & sapp::SAPP_MODIFIER_CTRL != 0 {
            key_mods.ctrl = true;
        }
        if value & sapp::SAPP_MODIFIER_ALT != 0 {
            key_mods.alt = true;
        }
        if value & sapp::SAPP_MODIFIER_SUPER != 0 {
            key_mods.logo = true;
        }

        key_mods
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl From<u32> for TouchPhase {
    fn from(event: u32) -> TouchPhase {
        match event {
            sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_BEGAN => TouchPhase::Started,
            sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_ENDED => TouchPhase::Ended,
            sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_CANCELLED => TouchPhase::Cancelled,
            sapp::sapp_event_type_SAPP_EVENTTYPE_TOUCHES_MOVED => TouchPhase::Moved,
            _ => unreachable!(),
        }
    }
}

/// A trait defining event callbacks.
pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context);
    fn draw(&mut self, _ctx: &mut Context);
    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn char_event(
        &mut self,
        _ctx: &mut Context,
        _character: char,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    /// Default implementation emulates mouse clicks
    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(ctx, x, y);
        }
    }

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    fn raw_mouse_motion(&mut self, _ctx: &mut Context, _dx: f32, _dy: f32) {}

    /// Window has been minimized
    /// Right now is only implemented on Android, and is called on a Pause ndk callback
    fn window_minimized_event(&mut self, _ctx: &mut Context) {}

    /// Window has been restored
    /// Right now is only implemented on Android, and is called on a Resume ndk callback
    fn window_restored_event(&mut self, _ctx: &mut Context) {}

    /// This event is sent when the userclicks the window's close button
    /// or application code calls the ctx.request_quit() function. The event
    /// handler callback code can handle this event by calling
    /// ctx.cancel_quit() to cancel the quit.
    /// If the event is ignored, the application will quit as usual.
    fn quit_requested_event(&mut self, _ctx: &mut Context) {}
}

/// A trait defining event callbacks.
/// Used for miniquad's setup with user-owned Context.
/// The only difference from EventHandler - will not receive "&mut Context"
pub trait EventHandlerFree {
    fn update(&mut self);
    fn draw(&mut self);
    fn resize_event(&mut self, _width: f32, _height: f32) {}
    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {}
    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {}
    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {}
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {}
    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}
    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {}
    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {}

    /// Default implementation emulates mouse clicks
    fn touch_event(&mut self, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(x, y);
        }
    }

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {}

    /// Window has been minimized
    /// Right now is only implemented on Android, and is called on a Pause ndk callback
    fn window_minimized_event(&mut self) {}

    /// Window has been restored
    /// Right now is only implemented on Android, and is called on a Resume ndk callback
    fn window_restored_event(&mut self) {}

    /// This event is sent when the userclicks the window's close button
    /// or application code calls the ctx.request_quit() function. The event
    /// handler callback code can handle this event by calling
    /// ctx.cancel_quit() to cancel the quit.
    /// If the event is ignored, the application will quit as usual.
    fn quit_requested_event(&mut self) {}
}
