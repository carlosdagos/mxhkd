use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Settings {
    /// The name of the program to use as a shell
    /// to execute commands
    pub shell: String,
    /// The key used to toggle modes
    pub mode_key: String,
    /// A command to run when the mode is switched
    pub mode_change_cmd: Option<String>,
    /// A command to run if a binding is not found
    pub not_found_cmd: Option<String>,
    /// The key used to toggle sticky
    /// mode
    pub sticky_mode: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub bindings: HashMap<String, String>,
}

#[cfg(test)]
mod config_tests {
    use toml;

    #[test]
    fn test_config_parse() {
        let config: crate::Config = toml::from_str(
            r#"
           [settings]
           shell = "shell"
           mode_key = "Escape"
           sticky_mode = "s"
           [bindings]
           foo = "foo"
           bar = "bar"
        "#,
        )
        .unwrap();

        assert_eq!(config.settings.shell, "shell")
    }
}
