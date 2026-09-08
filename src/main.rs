use std::time::Duration;
use std::thread;

use evdev::{AttributeSet, EventType, InputEvent, Key};
use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!"); // just to test if it loads correctly

    let mut keys = AttributeSet::<Key>::new();
    keys.insert(Key::BTN_LEFT);
    keys.insert(Key::BTN_RIGHT);

    let mut device = VirtualDeviceBuilder::new()?
        .name("rust-autoclicker")
        .with_keys(&keys)?
        .build()?;
    
    println!("Keys and Device set up.");

    // Insert a pause so the system can detect and initialize the new device.
    thread::sleep(Duration::from_millis(300));

    // Main loop
    loop{
        click(&mut device, Key::BTN_LEFT)?;
        thread::sleep(Duration::from_millis(500));
    }
}

// Click function
fn click(device: &mut VirtualDevice, key: Key) -> Result<(), Box<dyn std::error::Error>> {
        device.emit(&[InputEvent::new(EventType::KEY, key.0, 1,)])?;
        // 1 ms delay so it registers correctly.
        thread::sleep(Duration::from_millis(1));
        device.emit(&[InputEvent::new(EventType::KEY, key.0,0)])?;
    Ok(())
}