# Autoclicker

A lightweight UNIX autoclicker written in Rust with Wayland support, by using [evdev](https://docs.rs/evdev/latest/evdev/) for input handling.

## Building
*_Requires Rust_*

```bash
git clone https://github.com/Shikaru0/autoclicker.git
cd autoclicker
cargo build --release
```

## Configuring
In order for the keybind (hotkey) to work correctly, you need to set the correct device in the config.toml.
To find out which device is correct, install `evtest` or a different tool capable of monitoring input events. For this guide, we will be using `evtest`.

Then run:

```bash
sudo evtest
```

Look at the list of devices, find which event is your desired device (keyboard / mouse) and set the input_device_path event number to that number.
For instance, if our desired event = 0, we change our input_device_path from "/dev/input/event3" to "/dev/input/event0". 
This should not be done in app runtime to avoid conflicts.

## How does it work?

Wayland does not allow applications to click in other windows. To bypass this, it uses evdev to create a virtual device that behaves like a mouse. 
The autoclicker sends click events through this virtual device, using VirtualDevice.emit().

## Contributing
All contributions are greatly appreciated. Whether you want to fix a bug, improve existing functionality, add a new feature, or something else, feel free to contribute.

You can contribute by opening an issue, suggesting an idea, submitting a pull request, or you can personally dm me your contribution.