use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

use config::Config;
use evdev::KeyCode;
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub min_delay: i64,
    pub max_delay: i64,
    pub min_burst_delay: u64,
    pub max_burst_delay: u64,
    pub keybind: String,
    pub key: KeyCode,
    pub toggle: bool,
    pub input_device_path: String,
    pub click_count: u32,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    min_delay: i64,
    max_delay: i64,
    min_burst_delay: u64,
    max_burst_delay: u64,
    keybind: String,
    key: String,
    toggle: bool,
    input_device_path: String,
    click_count: u32,
}

pub fn config_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
    let app_config_dir = config_dir.join("rust-autoclicker");
    std::fs::create_dir_all(&app_config_dir)?;
    let config_file_path = app_config_dir.join("config.toml");

    if !config_file_path.exists() {
        let default_config = ConfigFile {
            min_delay: 100,
            max_delay: 115,
            min_burst_delay: 1,
            max_burst_delay: 3,
            keybind: "KEY_F1".to_string(),
            key: "left".to_string(),
            toggle: true,
            input_device_path: "/dev/input/event3".to_string(),
            click_count: 1,
        };

        let toml = toml::to_string_pretty(&default_config)?;
        std::fs::write(&config_file_path, toml)?;
    }

    Ok(config_file_path)
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let path = config_path()?;

    let settings = Config::builder()
        .add_source(config::File::from(path))
        .build()?;

    let min_delay = settings.get_int("min_delay")?;
    let max_delay = settings.get_int("max_delay")?;

    let min_burst_delay = settings.get_int("min_burst_delay").unwrap_or(1) as u64;
    let max_burst_delay = settings.get_int("max_burst_delay").unwrap_or(3) as u64;

    let input_device_path = settings.get_string("input_device_path")?;

    let keybind = settings.get_string("keybind")?;

    let key = match settings.get_string("key")?.as_str() {
        "left" => KeyCode::BTN_LEFT,
        "right" => KeyCode::BTN_RIGHT,
        _ => return Err("Use 'left' or 'right' for key input.".into()),
    };

    let toggle = settings.get_bool("toggle")?;

    let click_count = settings.get_int("click_count").unwrap_or(1) as u32;

    Ok(AppConfig {
        min_delay,
        max_delay,
        min_burst_delay,
        max_burst_delay,
        keybind,
        key,
        toggle,
        input_device_path,
        click_count,
    })
}

pub fn refresh_config(
    shared_config: &Arc<Mutex<AppConfig>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let new_config = load_config()?;
    let mut cfg = shared_config.lock().unwrap();
    *cfg = new_config;
    Ok(())
}

pub fn save_config(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let file_config = ConfigFile {
        min_delay: config.min_delay,
        max_delay: config.max_delay,
        min_burst_delay: config.min_burst_delay,
        max_burst_delay: config.max_burst_delay,
        keybind: config.keybind.clone(),
        key: match config.key {
            KeyCode::BTN_LEFT => "left".to_string(),
            KeyCode::BTN_RIGHT => "right".to_string(),
            _ => return Err("Invalid mouse button.".into()),
        },
        toggle: config.toggle,
        input_device_path: config.input_device_path.clone(),
        click_count: config.click_count,
    };

    let toml = toml::to_string_pretty(&file_config)?;
    std::fs::write(config_path()?, toml)?;

    Ok(())
}

pub fn watch_config(
    shared_config: Arc<Mutex<AppConfig>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path()?;
    let parent = path
        .parent()
        .ok_or("Config file has no parent directory")?
        .to_path_buf();

    let (tx, rx) = mpsc::channel();
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(&parent, RecursiveMode::NonRecursive)?;

    thread::spawn(move || {
        let _watcher = watcher;
        for res in rx.iter() {
            let Ok(event) = res else {
                continue;
            };

            if matches!(event.kind, EventKind::Access(_)) {
                continue;
            }

            if !event.paths.iter().any(|p| p == &path) {
                continue;
            }

            thread::sleep(Duration::from_millis(50));
            while rx.try_recv().is_ok() {}

            match refresh_config(&shared_config) {
                Ok(()) => println!("Config reloaded."),
                Err(e) => eprintln!("Config reload failed: {e}"),
            }
        }
    });

    Ok(())
}
