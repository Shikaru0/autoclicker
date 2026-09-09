use std::sync::{Arc, Mutex};

use evdev::KeyCode;
use config::Config;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct AppConfig{
    pub delay: i64,
    pub keybind: String,
    pub key: KeyCode,
    pub toggle: bool,
    pub input_device_path: String
}

#[derive(Serialize, Deserialize)]
struct ConfigFile{
    delay: i64,
    keybind: String,
    key: String,
    toggle: bool,
    input_device_path: String
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>>{
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    
    let delay = settings.get_int("delay")?;
    
    let input_device_path = settings.get_string("input_device_path")?;
    
    let keybind = settings.get_string("keybind")?;
    
    let key = match settings.get_string("key")?.as_str(){
        "left" => KeyCode::BTN_LEFT,
        "right" => KeyCode::BTN_RIGHT,
        _ => return Err("Use 'left' or 'right' for key input.".into()),
    };

    let toggle = settings.get_bool("toggle")?;

    Ok(AppConfig { delay, keybind, key, toggle, input_device_path })
}
    
pub fn refresh_config(shared_config: &Arc<Mutex<AppConfig>>) -> Result<(), Box<dyn std::error::Error>>{
    let new_config = load_config()?;
    let mut cfg = shared_config.lock().unwrap();
    *cfg = new_config;
    Ok(())
}

pub fn save_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>>{
    let file_config = ConfigFile{
        delay: config.delay,
        keybind: config.keybind.clone(),
        key: match config.key{
            KeyCode::BTN_LEFT => "left".to_string(),
            KeyCode::BTN_RIGHT => "right".to_string(),
            _ => return Err("Invalid mouse button.".into())
        },
        toggle: config.toggle,
        input_device_path: config.input_device_path.clone()
    };

    let toml = toml::to_string_pretty(&file_config)?;
    std::fs::write("config.toml", toml)?;

    Ok(())
}