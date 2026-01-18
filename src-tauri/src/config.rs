use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub pin: String,
    pub auto_start: bool,
    pub grid: GridConfig,
    pub buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    pub columns: u8,
    pub rows: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonConfig {
    pub id: String,
    pub label: String,
    pub position: ButtonPosition,
    pub action: ButtonAction,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub repeat: Option<RepeatConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatConfig {
    pub enabled: bool,
    #[serde(default = "default_repeat_interval")]
    pub interval_ms: u32,
}

fn default_repeat_interval() -> u32 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonPosition {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ButtonAction {
    Shortcut { keys: Vec<String> },
    TextAndEnter { text: String },
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: 52801,
            pin: String::new(),
            auto_start: false,
            grid: GridConfig {
                columns: 5,
                rows: 2,
            },
            buttons: vec![
                // 1行目
                ButtonConfig {
                    id: "compact".to_string(),
                    label: "Compact".to_string(),
                    position: ButtonPosition {
                        x: 0,
                        y: 0,
                        width: 2,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Meta".to_string(), "Shift".to_string(), "C".to_string()],
                    },
                    color: Some("#3B82F6".to_string()),
                    repeat: None,
                },
                ButtonConfig {
                    id: "tab".to_string(),
                    label: "Tab".to_string(),
                    position: ButtonPosition {
                        x: 2,
                        y: 0,
                        width: 1,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Tab".to_string()],
                    },
                    color: Some("#E5E7EB".to_string()),
                    repeat: None,
                },
                ButtonConfig {
                    id: "delete".to_string(),
                    label: "⌫".to_string(),
                    position: ButtonPosition {
                        x: 3,
                        y: 0,
                        width: 1,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Backspace".to_string()],
                    },
                    color: Some("#EF4444".to_string()), // 赤
                    repeat: Some(RepeatConfig {
                        enabled: true,
                        interval_ms: 80,
                    }),
                },
                ButtonConfig {
                    id: "accept".to_string(),
                    label: "Accept".to_string(),
                    position: ButtonPosition {
                        x: 4,
                        y: 0,
                        width: 1,
                        height: 2,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Return".to_string()],
                    },
                    color: Some("#F59E0B".to_string()),
                    repeat: None,
                },
                // 2行目
                ButtonConfig {
                    id: "new".to_string(),
                    label: "New".to_string(),
                    position: ButtonPosition {
                        x: 0,
                        y: 1,
                        width: 1,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Meta".to_string(), "N".to_string()],
                    },
                    color: Some("#E5E7EB".to_string()),
                    repeat: None,
                },
                ButtonConfig {
                    id: "esc".to_string(),
                    label: "ESC".to_string(),
                    position: ButtonPosition {
                        x: 1,
                        y: 1,
                        width: 1,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Escape".to_string()],
                    },
                    color: Some("#3B82F6".to_string()),
                    repeat: None,
                },
                ButtonConfig {
                    id: "mic".to_string(),
                    label: "🎤".to_string(),
                    position: ButtonPosition {
                        x: 2,
                        y: 1,
                        width: 1,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Meta".to_string(), "Shift".to_string(), "R".to_string()],
                    },
                    color: Some("#E5E7EB".to_string()),
                    repeat: None,
                },
                ButtonConfig {
                    id: "space".to_string(),
                    label: "Space".to_string(),
                    position: ButtonPosition {
                        x: 3,
                        y: 1,
                        width: 1,
                        height: 1,
                    },
                    action: ButtonAction::Shortcut {
                        keys: vec!["Space".to_string()],
                    },
                    color: Some("#E5E7EB".to_string()),
                    repeat: None,
                },
            ],
        }
    }
}

impl AppConfig {
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("swcc-controler");

        fs::create_dir_all(&config_dir).ok();
        config_dir.join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|content| serde_json::from_str(&content).ok())
                .unwrap_or_default()
        } else {
            let config = Self::default();
            config.save().ok();
            config
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
