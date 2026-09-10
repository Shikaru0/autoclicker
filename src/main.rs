mod config;
mod input;
mod click;

use clap::Parser;

use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

use evdev::{AttributeSet, KeyCode};
use evdev::uinput::{VirtualDevice};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command{
    Config{
        #[command(subcommand)]
        command: ConfigCommand
    }
}

#[derive(clap::Subcommand)]
enum ConfigCommand{
    Set{
        variable: String,
        value: String,
    },

    List
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match args.command{
        Some(Command::Config { command }) => {
            match command{
                ConfigCommand::Set{variable, value} => {
                    let mut config = config::load_config()?;

                    match variable.as_str(){
                        "min_delay" => {
                            config.min_delay = value.parse()?;
                        }
                        "max_delay" => {
                            config.max_delay = value.parse()?;
                        }

                        "keybind" => {
                            config.keybind = value;
                        }

                        "key" => {
                            config.key = match value.as_str(){
                                "left" => KeyCode::BTN_LEFT,
                                "right" => KeyCode::BTN_RIGHT,
                                _ => return Err("Invalid mouse key".into())
                            };
                        }

                        "toggle" => {
                            config.toggle = value.parse()?;
                        }

                        "input_device_path" => {
                            config.input_device_path = value;
                        }

                        _ => {
                            return Err({variable}.into());
                        }
                    }

                    config::save_config(&config)?;
                }
                ConfigCommand::List => {
                    let config = config::load_config()?;
                    println!("{config:?}");
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

    let mut device = VirtualDevice::builder()?
        .name("rust-autoclicker")
        .with_keys(&keys)?
        .build()?;

    println!("Keys and Device set up.");

    let key_held = Arc::new(AtomicBool::new(false));

    // Initialize input thread
    input::initialize(Arc::clone(&key_held), Arc::clone(&shared_config));

    // Insert a pause so the system can detect and initialize the new device.
    thread::sleep(Duration::from_millis(300));

    // Main loop
    loop{
        let (current_min_delay, current_max_delay, current_key) = {
            let cfg = shared_config.lock().unwrap();
            (cfg.min_delay, cfg.max_delay, cfg.key)
        };

        if key_held.load(Ordering::Relaxed){
            let delay = if current_min_delay != current_max_delay {
                rand::random_range(current_min_delay..current_max_delay)
            } else {
                current_min_delay
            };
            println!("Click delay: {delay}");
            click::click(&mut device, current_key)?;
            thread::sleep(Duration::from_millis(delay as u64));
        }
        else{
            thread::sleep(Duration::from_millis(10));
        }
    }
}