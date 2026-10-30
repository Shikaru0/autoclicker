use evdev::uinput::VirtualDevice;
use evdev::{EventType, InputEvent, KeyCode};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

pub fn click(
    device: &mut VirtualDevice,
    key: KeyCode,
    count: u32,
    min_burst_delay: u64,
    max_burst_delay: u64,
    total_clicks: &AtomicU32,
    max_clicks: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    for i in 0..count {
        if max_clicks != 0 && total_clicks.load(Ordering::Relaxed) >= max_clicks {
            return Ok(true);
        }

        device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 1)])?;
        // 1 ms delay so it registers correctly.
        thread::sleep(Duration::from_millis(1));
        device.emit(&[InputEvent::new(EventType::KEY.0, key.0, 0)])?;

        total_clicks.fetch_add(1, Ordering::Relaxed);

        if max_clicks != 0 && total_clicks.load(Ordering::Relaxed) >= max_clicks {
            return Ok(true);
        }

        if i + 1 < count {
            let delay = if min_burst_delay != max_burst_delay {
                rand::random_range(min_burst_delay..max_burst_delay)
            } else {
                min_burst_delay
            };

            thread::sleep(Duration::from_millis(delay))
        }
    }
    Ok(false)
}
