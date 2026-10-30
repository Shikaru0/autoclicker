use evdev::{Device, EventSummary, KeyCode};
use std::str::FromStr;
use std::sync::Condvar;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::config::AppConfig;

pub type HeldState = Arc<(Mutex<bool>, Condvar)>;

pub fn initialize(held_state: HeldState, config: Arc<Mutex<AppConfig>>) {
    thread::spawn(
        move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut current_device_path = {
                let cfg = config.lock().unwrap();
                cfg.input_device_path.clone()
            };

            let mut input = Device::open(&current_device_path)?;

            loop {
                let (current_keybind, current_toggle, new_device_path) = {
                    let cfg = config.lock().unwrap();
                    (
                        KeyCode::from_str(&cfg.keybind)?,
                        cfg.toggle,
                        cfg.input_device_path.clone(),
                    )
                };

                if new_device_path != current_device_path {
                    current_device_path = new_device_path;
                    match Device::open(&current_device_path) {
                        Ok(new_input) => {
                            input = new_input;
                        }
                        Err(e) => {
                            eprintln!("{:?}", e);
                            thread::sleep(Duration::from_millis(1000));
                        }
                    }
                }

                match input.fetch_events() {
                    Ok(events) => {
                        for event in events {
                            if let EventSummary::Key(_, k, state) = event.destructure()
                                && k == current_keybind
                            {
                                let (lock, cvar) = &*held_state;
                                let mut held = lock.lock().unwrap();

                                if current_toggle {
                                    if state == 1 {
                                        *held = !*held;
                                        cvar.notify_one();
                                    }
                                } else {
                                    match state {
                                        1 => {
                                            *held = true;
                                            cvar.notify_one();
                                        }
                                        0 => {
                                            *held = false;
                                            cvar.notify_one();
                                        }
                                        _ => {}
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
        },
    );
}
