mod click;
mod config;
mod input;

use clap::Parser;

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, KeyCode};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Manage application configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(clap::Subcommand)]
enum ConfigCommand {
    /// Set a value in config.toml to the given value, then saves the config
    Set { variable: String, value: String },

    /// Get the value of a configuration variable from config.toml
    Get { variable: String },

    /// Loads config from config.toml and prints (AppConfig) output
    List,

    /// Print the absolute path to config.toml
    Path,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command {
        Some(Command::Config { command }) => {
            match command {
                ConfigCommand::Set { variable, value } => {
                    let mut config = config::load_config()?;

                    match variable.as_str() {
                        "min_delay" => {
                            config.min_delay = value.parse()?;
                        }
                        "max_delay" => {
                            config.max_delay = value.parse()?;
                        }

                        "min_burst_delay" => {
                            config.min_burst_delay = value.parse()?;
                        }
                        "max_burst_delay" => {
                            config.max_burst_delay = value.parse()?;
                        }

                        "keybind" => {
                            config.keybind = value;
                        }

                        "key" => {
                            config.key = match value.as_str() {
                                "left" => KeyCode::BTN_LEFT,
                                "right" => KeyCode::BTN_RIGHT,
                                "extra" => KeyCode::BTN_EXTRA,
                                "middle" => KeyCode::BTN_MIDDLE,
                                "side" => KeyCode::BTN_SIDE,
                                "forward" => KeyCode::BTN_FORWARD,
                                "back" => KeyCode::BTN_BACK,
                                _ => return Err("Invalid mouse key".into()),
                            };
                        }

                        "toggle" => {
                            config.toggle = value.parse()?;
                        }

                        "input_device_path" => {
                            config.input_device_path = value;
                        }

                        "click_count" => {
                            let parsed: u32 = value.parse()?;
                            if parsed == 0 {
                                return Err("click_count must be at least 1".into());
                            }
                            config.click_count = parsed;
                        }

                        _ => {
                            return Err({ variable }.into());
                        }
                    }

                    config::save_config(&config)?;
                }
                ConfigCommand::Get { variable } => {
                    let config = config::load_config()?;

                    match variable.as_str() {
                        "min_delay" => println!("{}", config.min_delay),
                        "max_delay" => println!("{}", config.max_delay),
                        "min_burst_delay" => println!("{}", config.min_burst_delay),
                        "max_burst_delay" => println!("{}", config.max_burst_delay),
                        "keybind" => println!("{}", config.keybind),
                        "key" => match config.key {
                            KeyCode::BTN_LEFT => println!("left"),
                            KeyCode::BTN_RIGHT => println!("right"),
                            KeyCode::BTN_EXTRA => println!("extra"),
                            KeyCode::BTN_MIDDLE => println!("middle"),
                            KeyCode::BTN_SIDE => println!("side"),
                            KeyCode::BTN_FORWARD => println!("forward"),
                            KeyCode::BTN_BACK => println!("back"),
                            _ => return Err("Invalid mouse key in config".into()),
                        },
                        "toggle" => println!("{}", config.toggle),
                        "input_device_path" => println!("{}", config.input_device_path),
                        "click_count" => println!("{}", config.click_count),
                        _ => return Err(variable.into()),
                    }
                }
                ConfigCommand::List => {
                    let config = config::load_config()?;
                    println!("{config:?}");
                }
                ConfigCommand::Path => {
                    println!("{}", config::config_path()?.display());
                }
            }

            return Ok(());
        }

        None => {}
    }

    let shared_config = Arc::new(Mutex::new(config::load_config()?));

    let mut keys = AttributeSet::<KeyCode>::new();
    keys.insert(KeyCode::BTN_LEFT);
    keys.insert(KeyCode::BTN_RIGHT);
    keys.insert(KeyCode::BTN_EXTRA);
    keys.insert(KeyCode::BTN_MIDDLE);
    keys.insert(KeyCode::BTN_SIDE);
    keys.insert(KeyCode::BTN_FORWARD);
    keys.insert(KeyCode::BTN_BACK);

    let mut device = VirtualDevice::builder()?
        .name("rust-autoclicker")
        .with_keys(&keys)?
        .build()?;

    println!("Keys and Device set up.");

    let held_state: input::HeldState = Arc::new((Mutex::new(false), Condvar::new()));

    // Initialize input thread
    input::initialize(Arc::clone(&held_state), Arc::clone(&shared_config));

    // Watch config.toml
    config::watch_config(Arc::clone(&shared_config))?;

    // Insert a pause so the system can detect and initialize the new device.
    thread::sleep(Duration::from_millis(300));

    // Main loop
    loop {
        let (
            current_min_delay,
            current_max_delay,
            current_key,
            current_click_count,
            current_min_burst_delay,
            current_max_burst_delay,
        ) = {
            let cfg = shared_config.lock().unwrap();
            (
                cfg.min_delay,
                cfg.max_delay,
                cfg.key,
                cfg.click_count,
                cfg.min_burst_delay,
                cfg.max_burst_delay,
            )
        };

        let (lock, cvar) = &*held_state;
        let mut held = lock.lock().unwrap();

        if !*held {
            // Idle until cvar notifies
            while !*held {
                held = cvar.wait(held).unwrap();
            }
        }

        // Release mutex so input thread can flip the bool
        drop(held);

        let delay = if current_min_delay != current_max_delay {
            rand::random_range(current_min_delay..current_max_delay)
        } else {
            current_min_delay
        };
        println!("Click delay: {delay}");
        click::click(
            &mut device,
            current_key,
            current_click_count,
            current_min_burst_delay,
            current_max_burst_delay,
        )?;

        let mut held = lock.lock().unwrap();
        if *held {
            let (guard, _timeout) = cvar
                .wait_timeout(held, Duration::from_millis(delay as u64))
                .unwrap();
            held = guard;
        }
        drop(held);
    }
}
