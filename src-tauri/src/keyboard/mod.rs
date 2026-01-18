use crate::config::ButtonAction;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};

pub fn execute_action(action: &ButtonAction) -> Result<(), String> {
    match action {
        ButtonAction::Shortcut { keys } => execute_shortcut(keys),
        ButtonAction::TextAndEnter { text } => execute_text_and_enter(text),
    }
}

fn key_from_string(key: &str) -> Option<Key> {
    match key.to_lowercase().as_str() {
        // Modifiers
        "meta" | "cmd" | "command" | "super" => Some(Key::Meta),
        "control" | "ctrl" => Some(Key::Control),
        "alt" | "option" => Some(Key::Alt),
        "shift" => Some(Key::Shift),

        // Special keys
        "return" | "enter" => Some(Key::Return),
        "tab" => Some(Key::Tab),
        "space" | " " => Some(Key::Space),
        "backspace" => Some(Key::Backspace),
        "delete" | "forwarddelete" => Some(Key::Delete),
        "escape" | "esc" => Some(Key::Escape),

        // Arrow keys
        "up" | "arrowup" => Some(Key::UpArrow),
        "down" | "arrowdown" => Some(Key::DownArrow),
        "left" | "arrowleft" => Some(Key::LeftArrow),
        "right" | "arrowright" => Some(Key::RightArrow),

        // Function keys
        "f1" => Some(Key::F1),
        "f2" => Some(Key::F2),
        "f3" => Some(Key::F3),
        "f4" => Some(Key::F4),
        "f5" => Some(Key::F5),
        "f6" => Some(Key::F6),
        "f7" => Some(Key::F7),
        "f8" => Some(Key::F8),
        "f9" => Some(Key::F9),
        "f10" => Some(Key::F10),
        "f11" => Some(Key::F11),
        "f12" => Some(Key::F12),

        // Single character
        s if s.len() == 1 => {
            let c = s.chars().next().unwrap();
            Some(Key::Unicode(c))
        }

        _ => None,
    }
}

fn is_modifier(key: &Key) -> bool {
    matches!(key, Key::Meta | Key::Control | Key::Alt | Key::Shift)
}

fn execute_shortcut(keys: &[String]) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;

    // まず全てのモディファイアキーをリリースして状態をクリア
    let all_modifiers = [Key::Meta, Key::Control, Key::Alt, Key::Shift];
    for key in &all_modifiers {
        let _ = enigo.key(*key, Direction::Release);
    }
    std::thread::sleep(std::time::Duration::from_millis(10));

    let mut modifier_keys: Vec<Key> = Vec::new();
    let mut main_key: Option<Key> = None;

    for key_str in keys {
        if let Some(key) = key_from_string(key_str) {
            if is_modifier(&key) {
                modifier_keys.push(key);
            } else {
                main_key = Some(key);
            }
        } else {
            return Err(format!("Unknown key: {}", key_str));
        }
    }

    let main_key = main_key.ok_or("No main key specified")?;

    // モディファイアがない場合は単純にClickを使う
    if modifier_keys.is_empty() {
        enigo.key(main_key, Direction::Click)
            .map_err(|e| format!("Failed to click key: {:?}", e))?;
        return Ok(());
    }

    // Press modifiers
    for key in &modifier_keys {
        enigo.key(*key, Direction::Press)
            .map_err(|e| format!("Failed to press modifier: {:?}", e))?;
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    // Press and release main key
    enigo.key(main_key, Direction::Click)
        .map_err(|e| format!("Failed to click key: {:?}", e))?;
    std::thread::sleep(std::time::Duration::from_millis(20));

    // Release modifiers in reverse order
    for key in modifier_keys.iter().rev() {
        enigo.key(*key, Direction::Release)
            .map_err(|e| format!("Failed to release modifier: {:?}", e))?;
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    Ok(())
}

fn execute_text_and_enter(text: &str) -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;

    // Type text
    enigo.text(text)
        .map_err(|e| format!("Failed to type text: {:?}", e))?;

    // Press Enter
    enigo.key(Key::Return, Direction::Click)
        .map_err(|e| format!("Failed to press Enter: {:?}", e))?;

    Ok(())
}
