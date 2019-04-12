use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Window,
    Normal,
    NormalSticky,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum KeyState {
    Normal,
    Shift,
    Ctrl,
    Alt,
    Super,
}

impl KeyState {
    pub fn from_xcb(state: u16) -> KeyState {
        println!("Saw state: {:?}", state);

        match state {
            // Shift pressed (even if Caps is on)
            0x1 => KeyState::Shift,
            0x3 => KeyState::Shift,
            // Ctrl
            0x4 => KeyState::Ctrl,
            // Alt
            0x8 => KeyState::Alt,
            // Super
            0x40 => KeyState::Super,
            // Anything else is ignored
            _ => KeyState::Normal,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    ModeToggleKey,
    SomeKey {
        key: u8,
        state: KeyState,
        string: Option<String>,
    },
}
