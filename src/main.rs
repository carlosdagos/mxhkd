use std::fs;
use xcb;

use crate::config::Config;
use crate::types::{Key, KeyState, Mode};
use clap::{App, Arg};

mod config;
mod types;
mod xcb_util;
mod xmodmap;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Runs the command specified about the mode
/// supplied.
fn notify_mode(shell: &str, cmd: &str, mode: Mode) {
    let mode_str = format!("{:?}", mode);
    let cmd = cmd.replace("%{mode}%", &mode_str);
    xcb_util::run_command(&shell, &cmd);
}

/// Runs a command to notify a binding
/// was not found
fn notify_binding_not_found(shell: &str, cmd: &str, binding: &str) {
    let cmd = cmd.replace("%{binding}%", &binding);
    xcb_util::run_command(&shell, &cmd);
}

/// Utility function to switch to window mode.
/// Frees the keyboard if `free_keyboard` is set.
fn switch_to_window(
    conn: &xcb::Connection,
    screen: xcb::Window,
    xmodmap: &xmodmap::XModMap,
    config: &Config,
    free_keyboard: bool,
) {
    println!("Switching to Window mode. Listening only on mode key.");

    if free_keyboard {
        println!("Freeing keyboard.");
        xcb_util::free_keyboard(conn);
    }

    let mode_key = xmodmap.get_key(&config.settings.mode_key).unwrap();
    xcb_util::grab_key(&conn, screen, *mode_key);

    if let Some(cmd) = &config.settings.mode_change_cmd {
        notify_mode(&config.settings.shell, cmd, Mode::Window);
    }
}

/// Utility function to switch to normal mode.
/// Grabs the entire keyboard.
fn switch_to_normal(conn: &xcb::Connection, screen: xcb::Window, config: &Config) {
    println!("Switching to Normal mode. Grabbing the entire keyboard.");
    xcb_util::grab_keyboard(conn, screen);

    if let Some(cmd) = &config.settings.mode_change_cmd {
        notify_mode(&config.settings.shell, cmd, Mode::Normal);
    }
}

/// Sleeps until a key press event arrives to the connection.
fn on_read_key_press<F>(conn: &xcb::Connection, mut f: F)
where
    F: FnMut(&xcb::KeyPressEvent) -> (),
{
    let event = conn.wait_for_event();
    match event {
        None => {
            panic!("No event could be read from the connection. Disconnected?");
        }
        Some(event) => {
            let r = event.response_type();
            if r == xcb::KEY_PRESS as u8 {
                let key_press: &xcb::KeyPressEvent = unsafe { xcb::cast_event(&event) };
                f(key_press);
            }
        }
    }
}

/// Sleeps until a known key press event happens.
fn on_key<F>(conn: &xcb::Connection, modmap: &xmodmap::XModMap, config: &Config, mut f: F)
where
    F: FnMut(Key) -> (),
{
    on_read_key_press(conn, |key_press| {
        let key = xcb_util::key_for(&key_press, modmap, config);
        f(key);
    })
}

enum ListenResult {
    RunCmd { cmd: String },
    SwitchMode,
    SwitchSticky,
    NoMatch { binding: String },
}

use ListenResult::*;

/// Grabs the connection and starts listening for key sequences.
/// If the setting `key_press_delay` is exceeded then this function returns.
/// Returns an Option with a command to be run.
fn listen_for_key_sequence(
    conn: &xcb::Connection,
    modmap: &xmodmap::XModMap,
    initial_key: Key,
    config: &Config,
) -> ListenResult {
    let string = match initial_key {
        Key::SomeKey {
            key: _,
            state: _,
            string,
        } => string,
        _ => panic!("Invalid state encountered."),
    };

    let mut current_chain = match string {
        None => String::from(""),
        Some(initial) => initial,
    };

    let mut should_beak = false;
    let mut should_switch = false;

    if Some(current_chain.to_string()) == config.settings.sticky_mode {
        return SwitchSticky;
    }

    loop {
        let available_commands = config
            .bindings
            .iter()
            .filter(|&(k, _)| k.starts_with(&current_chain));

        println!("Current chain: {}", current_chain);
        if let Some(cmd) = config.bindings.get(&current_chain) {
            // We found a candidate
            return RunCmd {
                cmd: cmd.to_string(),
            };
        } else if available_commands.count() == 0 {
            // We found no commands
            return NoMatch {
                binding: current_chain,
            };
        }

        on_key(conn, modmap, config, |key_press| {
            match key_press {
                Key::SomeKey {
                    key: _,
                    state,
                    string,
                } => {
                    // Possibly pressing a modifier key. Careful here.
                    if state == KeyState::Normal && string == None {
                        return;
                    }

                    match string {
                        // Not detected by xmodmap, bail.
                        None => should_beak = true,
                        Some(mapped_string) => {
                            current_chain = format!("{}{}", current_chain, mapped_string);
                        }
                    }
                }
                Key::ModeToggleKey => {
                    should_switch = true;
                }
            }
        });

        if should_switch {
            return SwitchMode;
        }

        if should_beak {
            return NoMatch {
                binding: current_chain,
            };
        }
        // Else keep looping, there may
        // still be a command in there
    }
}

/// Process a key event.
fn process_key_press(
    key_type: Key,
    conn: &xcb::Connection,
    screen: xcb::Window,
    config: &Config,
    modmap: &xmodmap::XModMap,
    mode: &mut Mode,
) {
    match key_type {
        Key::SomeKey {
            key,
            state: _,
            string: _,
        } => {
            let cmd = listen_for_key_sequence(conn, modmap, key_type, config);
            match cmd {
                RunCmd { cmd } => {
                    println!("Found command to run: {}", cmd);

                    if *mode == Mode::Normal {
                        println!("Going back to Window mode");
                        *mode = Mode::Window;
                        switch_to_window(conn, screen, modmap, config, true);
                    }

                    xcb_util::run_command(&config.settings.shell, &cmd);
                }
                SwitchMode => {
                    println!("Triggered switch key whilst listenting to command.");

                    *mode = Mode::Window;
                    switch_to_window(conn, screen, modmap, config, true);
                }
                SwitchSticky => {
                    println!("Triggered sticky mode.");
                    *mode = Mode::NormalSticky;
                }
                NoMatch { binding } => {
                    println!(
                        "Key '{}' exited the command listener. No candidates left?",
                        key
                    );

                    if let Some(cmd) = &config.settings.not_found_cmd {
                        notify_binding_not_found(&config.settings.shell, &cmd, &binding);
                    }

                    if *mode == Mode::Normal {
                        println!("Going back to Window mode");
                        *mode = Mode::Window;
                        switch_to_window(conn, screen, modmap, config, true);
                    }
                }
            }
        }
        Key::ModeToggleKey => {
            // I think the only possible case here is
            // to switch to Normal?
            xcb_util::toggle_mode(mode);
            println!("Mode toggled to: {:?}", mode);

            if *mode == Mode::Window {
                switch_to_window(conn, screen, modmap, config, true);
            } else if *mode == Mode::Normal {
                switch_to_normal(conn, screen, config);
            }
        }
    }
}

fn main() {
    let args = App::new(APP_NAME)
        .version(APP_VERSION)
        .about(APP_DESCRIPTION)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets the configuration file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let config = args.value_of("config").unwrap();
    let config = fs::read_to_string(config).expect("Could not read config file.");
    let config: Config = toml::from_str(config.as_str()).expect("Could not parse config file.");

    let (conn, screen) = xcb_util::setup_connection();

    let modmap = xmodmap::parse_xmodmap();

    // Start in Window mode by default. Almost no effects should
    // be perceived by this point
    let mut mode = Mode::Window;
    println!("Starting in mode: {:?}.", mode);

    // Keyboard is not grabbed, so no need to free it
    switch_to_window(&conn, screen, &modmap, &config, false);

    loop {
        on_read_key_press(&conn, |key_press| {
            let key = xcb_util::key_for(key_press, &modmap, &config);
            process_key_press(key, &conn, screen, &config, &modmap, &mut mode);
        });
    }
}
