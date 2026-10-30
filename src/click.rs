use evdev::uinput::VirtualDevice;
use evdev::{EventType, InputEvent, KeyCode};
use std::thread;
use std::time::Duration;

pub fn click(device: &mut VirtualDevice, key: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
    device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 1)])?;
    // 1 ms delay so it registers correctly.
    thread::sleep(Duration::from_millis(1));
    device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 0)])?;
    Ok(())
}
