use std::{collections::HashMap, fs, path::Path};

use anyhow::{bail, Context, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;

const DEFAULT: &str = include_str!("../config/keymap.toml");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Action {
    Quit,
    SwitchPane,
    Up,
    Down,
    Open,
    Parent,
    Refresh,
}

impl Action {
    fn parse(name: &str) -> Result<Self> {
        match name {
            "quit" => Ok(Self::Quit),
            "switch_pane" => Ok(Self::SwitchPane),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "open" => Ok(Self::Open),
            "parent" => Ok(Self::Parent),
            "refresh" => Ok(Self::Refresh),
            _ => bail!("unknown keymap action '{name}'"),
        }
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    #[serde(default)]
    bindings: HashMap<String, Vec<String>>,
    #[serde(default)]
    planned: HashMap<String, Vec<String>>,
}

pub struct Keymap {
    bindings: HashMap<(KeyCode, KeyModifiers), Action>,
    labels: HashMap<Action, String>,
}

impl Keymap {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let mut config: Config = toml::from_str(DEFAULT).context("invalid bundled keymap")?;
        let override_path = path.map(Path::to_path_buf).or_else(|| {
            std::env::var_os("HOME").map(|home| {
                Path::new(&home)
                    .join(".config")
                    .join("rucommander")
                    .join("keymap.toml")
            })
        });
        if let Some(ref config_path) = override_path {
            if path.is_some() || config_path.exists() {
                let contents = fs::read_to_string(config_path)
                    .with_context(|| format!("reading keymap {}", config_path.display()))?;
                let overrides: Config = toml::from_str(&contents)
                    .with_context(|| format!("parsing keymap {}", config_path.display()))?;
                config.bindings.extend(overrides.bindings);
                // Planned entries document future actions and are not dispatched.
                config.planned.extend(overrides.planned);
            }
        }
        Self::from_config(config)
    }

    fn from_config(config: Config) -> Result<Self> {
        let mut bindings = HashMap::new();
        let mut labels = HashMap::new();
        for (name, keys) in config.bindings {
            let action = Action::parse(&name)?;
            if let Some(first) = keys.first() {
                labels.insert(action, first.clone());
            }
            for text in keys {
                let key = parse_key(&text).with_context(|| format!("{name}: {text}"))?;
                if let Some(previous) = bindings.insert(key, action) {
                    bail!("key '{text}' is assigned to both {previous:?} and {action:?}");
                }
            }
        }
        Ok(Self { bindings, labels })
    }

    pub fn action(&self, event: KeyEvent) -> Option<Action> {
        self.bindings.get(&(event.code, event.modifiers)).copied()
    }

    pub fn label(&self, action: Action) -> &str {
        self.labels.get(&action).map(String::as_str).unwrap_or("—")
    }
}

fn parse_key(text: &str) -> Result<(KeyCode, KeyModifiers)> {
    let mut modifiers = KeyModifiers::NONE;
    let mut parts = text.split('+').peekable();
    while let Some(part) = parts.next() {
        if parts.peek().is_none() {
            let code = match part.to_ascii_lowercase().as_str() {
                "up" => KeyCode::Up,
                "down" => KeyCode::Down,
                "left" => KeyCode::Left,
                "right" => KeyCode::Right,
                "enter" => KeyCode::Enter,
                "backspace" => KeyCode::Backspace,
                "tab" => KeyCode::Tab,
                "esc" => KeyCode::Esc,
                "insert" => KeyCode::Insert,
                name if name.starts_with('f') && name.len() > 1 => match name[1..].parse::<u8>() {
                    Ok(number @ 1..=12) => KeyCode::F(number),
                    _ => bail!("invalid key '{text}'"),
                },
                _ => match part.chars().collect::<Vec<_>>().as_slice() {
                    [character] => KeyCode::Char(if modifiers.contains(KeyModifiers::SHIFT) {
                        character.to_ascii_uppercase()
                    } else {
                        character.to_ascii_lowercase()
                    }),
                    _ => bail!("invalid key '{text}'"),
                },
            };
            return Ok((code, modifiers));
        }
        let modifier = match part.to_ascii_lowercase().as_str() {
            "ctrl" => KeyModifiers::CONTROL,
            "alt" => KeyModifiers::ALT,
            "shift" => KeyModifiers::SHIFT,
            _ => bail!("invalid modifier in '{text}'"),
        };
        if modifiers.contains(modifier) {
            bail!("duplicate modifier in '{text}'");
        }
        modifiers |= modifier;
    }
    bail!("empty key binding")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bindings_and_planned_keys() {
        let map = Keymap::from_config(toml::from_str(DEFAULT).unwrap()).unwrap();
        assert_eq!(
            map.action(KeyEvent::from(KeyCode::F(10))),
            Some(Action::Quit)
        );
        assert_eq!(map.action(KeyEvent::from(KeyCode::F(5))), None);
        assert_eq!(
            map.action(KeyEvent::from(KeyCode::Tab)),
            Some(Action::SwitchPane)
        );
        assert_eq!(
            map.action(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL)),
            Some(Action::Refresh)
        );
    }

    #[test]
    fn override_replaces_one_action_without_removing_others() {
        let mut config: Config = toml::from_str(DEFAULT).unwrap();
        let custom: Config = toml::from_str("[bindings]\nquit = ['Ctrl+X']").unwrap();
        config.bindings.extend(custom.bindings);
        let map = Keymap::from_config(config).unwrap();
        assert_eq!(map.action(KeyEvent::from(KeyCode::F(10))), None);
        assert_eq!(
            map.action(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL)),
            Some(Action::Quit)
        );
        assert_eq!(
            map.action(KeyEvent::from(KeyCode::Tab)),
            Some(Action::SwitchPane)
        );
    }

    #[test]
    fn duplicate_keys_and_invalid_bindings_fail() {
        let config: Config =
            toml::from_str("[bindings]\nquit = ['Tab']\nswitch_pane = ['Tab']").unwrap();
        assert!(Keymap::from_config(config).is_err());
        assert!(parse_key("Ctrl+NotAKey").is_err());
        assert!(parse_key("F13").is_err());
    }
}
