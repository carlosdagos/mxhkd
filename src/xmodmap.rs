use bimap::BiMap;
use regex::Regex;
use std::process::Command;

/// Holds the xmodmap parsed results
pub struct XModMap {
    mappings: BiMap<u8, String>,
}

impl XModMap {
    pub fn new() -> XModMap {
        XModMap {
            mappings: BiMap::new(),
        }
    }

    pub fn get_string(&self, c: &u8) -> Option<&String> {
        self.mappings.get_by_left(c)
    }

    pub fn get_key(&self, c: &String) -> Option<&u8> {
        self.mappings.get_by_right(c)
    }

    pub fn set(&mut self, code: u8, s: String) {
        self.mappings.insert(code, s);
    }
}

/// Runs xmodmap to get a real sense of how the client
/// keyboard is mapped out
pub fn parse_xmodmap() -> XModMap {
    println!("Using xmodmap to detect keyboard layout.");

    let output = Command::new("xmodmap")
        .arg("-pke")
        .output()
        .expect("Run xmodmap. Is it installed?");

    let lines = String::from_utf8(output.stdout).expect("Read xmodmap output");
    let lines = lines.split("\n");

    parse_modmap_lines(lines)
}

pub fn parse_modmap_lines<L, S>(lines: L) -> XModMap
where
    S: AsRef<str>,
    L: Iterator<Item = S>,
{
    let mut mappings = XModMap::new();

    // This regex (ugh...)
    // Note: This basically controls which keys we'll listen to in the entire program
    // Note: Honestly I don't know how long it will take to get this one right
    let re = Regex::new(r"keycode\s+(\d+) = ([a-z]|\d|F[0-9]+|Escape|Tab|period|comma)\s").unwrap();

    for line in lines {
        let line = line.as_ref();
        let captures = re.captures(line);
        match captures {
            None => continue,
            Some(c) => {
                let code = u8::from_str_radix(&c[1], 10).unwrap();
                let key = c[2].to_string();

                mappings.set(code, key);
            }
        }
    }

    return mappings;
}

#[cfg(test)]
mod modmap_test {
    use crate::xmodmap::*;

    #[test]
    fn modmap_output_parse() {
        // Sample (shortened) output of `xmodmap -pke`
        let sample_output = vec![
            "keycode   8 =",
            "keycode   9 = Escape NoSymbol Escape",
            "keycode  10 = 1 exclam 1 exclam",
            "keycode  11 = 2 at 2 at",
            "keycode  22 = BackSpace BackSpace BackSpace BackSpace",
            "keycode  23 = Tab ISO_Left_Tab Tab ISO_Left_Tab",
            "keycode  24 = q Q q Q",
            "keycode  48 = apostrophe quotedbl apostrophe",
            "keycode  60 = period greater period greater",
            "keycode  75 = F9 F9 F9 F9 F9 F9 XF86Switch_VT_9",
            "keycode  76 = F10 F10 F10 F10 F10 F10 XF86Switch_VT_10",
        ];

        let sample_output = sample_output.iter();

        let mappings = parse_modmap_lines(sample_output);

        assert_eq!(mappings.get_string(&8), None);
        assert_eq!(mappings.get_string(&48), None);
        assert_eq!(mappings.get_string(&9), Some(&"Escape".to_string()));
        assert_eq!(mappings.get_string(&10), Some(&"1".to_string()));
        assert_eq!(mappings.get_string(&11), Some(&"2".to_string()));
        assert_eq!(mappings.get_string(&23), Some(&"Tab".to_string()));
        assert_eq!(mappings.get_string(&60), Some(&"period".to_string()));
        assert_eq!(mappings.get_string(&75), Some(&"F9".to_string()));
        assert_eq!(mappings.get_string(&76), Some(&"F10".to_string()));
    }
}
