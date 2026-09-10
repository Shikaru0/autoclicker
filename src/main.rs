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
    Load,

    Set{
        variable: String,
        value: String,
    },

    Refresh,

    List
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("main.rs initialized"); // just to test if it loads correctly

    let args = Cli::parse();

    match args.command{
        Some(Command::Config { command }) => {
            match command{
                ConfigCommand::Load => {
                    let config = config::load_config()?;
                    println!("{config:?}");
                }
                ConfigCommand::Set{variable, value} => {
                    let mut config = config::load_config()?;

                    match variable.as_str(){
                        "delay" => {
                            config.delay = value.parse()?;
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
                ConfigCommand::Refresh => {
                    let shared_config = Arc::new(Mutex::new(config::load_config()?));
                    config::refresh_config(&shared_config)?; 
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

    // Quick test to see if config saving/loading/refreshing works. (for this to work, make sure that in cfg.delay = x, x is different from delay in config.toml)
    /*
    {
        let mut cfg = shared_config.lock().unwrap();
        cfg.delay = 300;
    }

    println!("pre save config: {:?}", std::fs::read_to_string("config.toml")?);

    config::save_config(&shared_config.lock().unwrap())?;
    config::refresh_config(&shared_config)?;

    println!("post save config: {:?}", std::fs::read_to_string("config.toml")?);
    */

    // Main loop
    loop{
        let (current_delay, current_key) = {
            let cfg = shared_config.lock().unwrap();
            (cfg.delay, cfg.key)
        };

        if key_held.load(Ordering::Relaxed){
            click::click(&mut device, current_key)?;
            thread::sleep(Duration::from_millis(current_delay as u64));
        }
        else{
            //refresh_config(&shared_config); // to test if it works
            thread::sleep(Duration::from_millis(10));
        }
    }
}