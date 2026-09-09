mod config;
mod input;
mod click;

use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};

use evdev::{AttributeSet, KeyCode};
use evdev::uinput::{VirtualDevice};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main.rs initialized"); // just to test if it loads correctly

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