use std::str::FromStr;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use evdev::{Device, EventSummary, KeyCode};

use crate::config::AppConfig;

pub fn initialize(key_held: Arc<AtomicBool>, config: Arc<Mutex<AppConfig>>){
    thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut current_device_path = {
            let cfg = config.lock().unwrap();
            cfg.input_device_path.clone()
        };

        let mut input = Device::open(&current_device_path)?;

        loop{
            let (current_keybind, current_toggle, new_device_path) = {
                let cfg = config.lock().unwrap();
                (KeyCode::from_str(&cfg.keybind)?, cfg.toggle, cfg.input_device_path.clone())
            };

            if new_device_path != current_device_path{
                current_device_path = new_device_path;
                match Device::open(&current_device_path){
                    Ok(new_input) => {
                        input = new_input;
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        thread::sleep(Duration::from_millis(1000));
                    }
                }
            }

            match input.fetch_events(){
                Ok(events) => {
                    for event in events{
                        if let EventSummary::Key(_, k, state) = event.destructure() {
                            if k == current_keybind {
                                if current_toggle{
                                    if state == 1{
                                        let _ = key_held.fetch_xor(true, Ordering::Relaxed);
                                    }
                                }
                                else{
                                    match state{
                                        1 => {
                                            key_held.store(true, Ordering::Relaxed);
                                        }
                                        0 => {
                                            key_held.store(false, Ordering::Relaxed);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }            
                    }
                }
                Err(e) => {
                    eprint!("{e}");
                    thread::sleep(Duration::from_millis(1000));
                }
            }
        }
    });
}