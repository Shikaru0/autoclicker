use evdev::uinput::VirtualDevice;
use evdev::{EventType, InputEvent, KeyCode};
use std::thread;
use std::time::Duration;

pub fn click(
    device: &mut VirtualDevice,
    key: KeyCode,
    count: u32,
    min_burst_delay: u64,
    max_burst_delay: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..count {
        device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 1)])?;
        // 1 ms delay so it registers correctly.
        thread::sleep(Duration::from_millis(1));
        device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 0)])?;

        if i + 1 < count {
            let delay = if min_burst_delay != max_burst_delay {
                rand::random_range(min_burst_delay..max_burst_delay)
            } else {
                min_burst_delay
            };

            thread::sleep(Duration::from_millis(delay))
        }
    }
    Ok(())
}
