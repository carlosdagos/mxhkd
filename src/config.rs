use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ModKeySpec {
    pub key: String,
    pub modifier: Option<KeyState>,
}

#[derive(Deserialize)]
pub struct Settings {
    /// The name of the program to use as a shell
    /// to execute commands
    pub shell: String,
    /// The key used to toggle modes
    pub mode_switch: ModKeySpec,
    /// A command to run when the mode is switched
    pub mode_change_cmd: Option<String>,
    /// A command to run if a binding is not found
    pub not_found_cmd: Option<String>,
    /// The key used to toggle sticky
    /// mode
    pub sticky_mode: Option<String>,
}

impl Settings {
    pub fn is_mode_key(&self, k: &str, m: &KeyState) -> bool {
        if let Some(modifier) = &self.mode_switch.modifier {
            self.mode_switch.key == k && modifier == m
        } else {
            self.mode_switch.key == k
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub bindings: HashMap<String, String>,
}

#[cfg(test)]
mod config_tests {
    use crate::types::*;
    use toml;

    #[test]
    fn test_config_parse_simple_mode_key() {
        let config: crate::Config = toml::from_str(
            r#"
           [settings]
           shell = "shell"
           mode_switch = { key = "Escape" } 
           sticky_mode = "s"
           [bindings]
           foo = "foo"
           bar = "bar"
        "#,
        )
        .unwrap();

        assert_eq!(config.settings.shell, "shell");
        assert_eq!(config.settings.mode_switch.key, "Escape");
        assert_eq!(config.settings.mode_switch.modifier, None);
    }

    #[test]
    fn test_config_parse_chorded_mode_key() {
        let config: crate::Config = toml::from_str(
            r#"
           [settings]
           shell = "shell"
           mode_switch = { key = "space", modifier = "Alt" }
           sticky_mode = "s"
           [bindings]
           foo = "foo"
           bar = "bar"
        "#,
        )
        .unwrap();

        assert_eq!(config.settings.shell, "shell");
        assert_eq!(config.settings.mode_switch.key, "space");
        assert_eq!(config.settings.mode_switch.modifier, Some(KeyState::Alt))
    }
}
