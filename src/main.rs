use std::str::FromStr;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use evdev::{AttributeSet, Device, EventSummary, EventType, InputEvent, KeyCode};
use evdev::uinput::{VirtualDevice};

use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("main.rs initialized"); // just to test if it loads correctly

    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;

    let delay = settings.get_int("delay")?;

    let input_device_path = settings.get_string("input_device_path")?;

    let key_string = settings.get_string("keybind")?;

    let keybind = KeyCode::from_str(&key_string)?;

    let key = match settings.get_string("key")?.as_str(){
        "left" => KeyCode::BTN_LEFT,
        "right" => KeyCode::BTN_RIGHT,
        _ => return Err("Use 'left' or 'right' for key input.".into()),
    };

    let toggle = settings.get_bool("toggle")?;

    let mut keys = AttributeSet::<KeyCode>::new();
    keys.insert(KeyCode::BTN_LEFT);
    keys.insert(KeyCode::BTN_RIGHT);

    let mut device = VirtualDevice::builder()?
        .name("rust-autoclicker")
        .with_keys(&keys)?
        .build()?;

    println!("Keys and Device set up.");

    let key_held = Arc::new(AtomicBool::new(false));
    let key_held_input = Arc::clone(&key_held);

    // Input thread
    thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut input = Device::open(&input_device_path)?;

        loop{
            match input.fetch_events(){
                Ok(events) => {
                    for event in events{
                        if let EventSummary::Key(_, k, state) = event.destructure() {
                            if k == keybind {
                                if toggle{
                                    if state == 1{
                                        let _ = key_held_input.fetch_xor(true, Ordering::Relaxed);
                                    }
                                }
                                else{
                                    match state{
                                        1 => {
                                            key_held_input.store(true, Ordering::Relaxed);
                                        }
                                        0 => {
                                            key_held_input.store(false, Ordering::Relaxed);
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

    // Insert a pause so the system can detect and initialize the new device.
    thread::sleep(Duration::from_millis(300));

    // Main loop
    loop{
        if key_held.load(Ordering::Relaxed){
            click(&mut device, key)?;
            thread::sleep(Duration::from_millis(delay as u64));
        }
        else{
            thread::sleep(Duration::from_millis(10));
        }
    }
}

// Click function
fn click(device: &mut VirtualDevice, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
        device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 1,)])?;
        // 1 ms delay so it registers correctly.
        thread::sleep(Duration::from_millis(1));
        device.emit(&[InputEvent::new(EventType::KEY.0, key.0,0)])?;
    Ok(())
}