use std::io::{self, Write};
use std::iter::Iterator;
use std::process::Command;
use xcb;

use crate::config::Config;
use crate::types::*;
use crate::xmodmap::XModMap;

/// Setup connection
pub fn setup_connection() -> (xcb::Connection, xcb::Window) {
    let (conn, _screen_num) = xcb::Connection::connect_with_xlib_display().unwrap();

    let setup = conn.get_setup();
    let screen = setup.roots().next().unwrap().root();
    conn.flush();

    (conn, screen)
}

/// Grab the whole keyboard
pub fn grab_keyboard(conn: &xcb::Connection, screen: xcb::Window) {
    let cook = xcb::grab_keyboard(
        conn,
        true,
        screen,
        xcb::CURRENT_TIME,
        xcb::GRAB_MODE_ASYNC as u8,
        xcb::GRAB_MODE_ASYNC as u8,
    );
    match cook.get_reply() {
        Ok(r) => {
            if r.status() != xcb::GRAB_STATUS_SUCCESS as u8 {
                panic!("could not grab keyboard, got reply: {}", r.status())
            }
        }
        Err(e) => panic!("could not grab keyboard: {:?}", e),
    }
}

/// Free the whole keyboard
pub fn free_keyboard(conn: &xcb::Connection) {
    let cook = xcb::ungrab_keyboard(conn, xcb::CURRENT_TIME);
    match cook.request_check() {
        Ok(_) => (),
        Err(e) => panic!("Could not free keyboard: {:?}.", e),
    }
}

/// Only grabs the specified key
pub fn grab_key(conn: &xcb::Connection, screen: xcb::Window, key: u8) {
    let cook = xcb::grab_key(
        &conn,
        true,
        screen,
        xcb::MOD_MASK_ANY as u16,
        key,
        xcb::GRAB_MODE_ASYNC as u8,
        xcb::GRAB_MODE_SYNC as u8,
    );
    match cook.request_check() {
        Ok(_) => (),
        Err(e) => panic!("Could not grab the mode key: {:?}", e),
    }
}

/// Toggles the mode
pub fn toggle_mode(mode: &mut Mode) {
    match mode {
        Mode::Normal => *mode = Mode::Window,
        Mode::NormalSticky => *mode = Mode::Window,
        Mode::Window => *mode = Mode::Normal,
    }
}

/// Get the details for a key
pub fn key_for(key_event: &xcb::KeyPressEvent, xmodmap: &XModMap, config: &Config) -> Key {
    let key = key_event.detail();
    let state = KeyState::from_xcb(key_event.state());
    let string = xmodmap.get_string(&key);

    let is_mode_key = if let Some(s) = string {
        *s == config.settings.mode_key
    } else {
        false
    };

    if is_mode_key {
        return Key::ModeToggleKey;
    } else {
        if state == KeyState::Shift {
            let string = string.map(|s| s.to_uppercase());
            return Key::SomeKey { key, state, string };
        } else {
            let string = string.map(|s| s.to_string());
            return Key::SomeKey { key, state, string };
        }
    }
}

/// Runs a command
pub fn run_command(shell: &str, cmd: &str) {
    println!("Running command: {}", cmd);
    match Command::new(shell).arg("-c").arg(cmd).output() {
        Ok(out) => {
            if !out.status.success() {
                println!("No success whilst running '{}'.", cmd);
                let _ = io::stdout().write_all(&out.stderr);
            }
        }
        Err(e) => println!("Error running {}. Error: {:?}.", cmd, e),
    }
}
