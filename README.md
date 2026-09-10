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

### Where is config.toml?
The config.toml is auto-generated in the [config_dir](https://docs.rs/dirs/latest/dirs/fn.config_dir.html) by [dirs](https://docs.rs/dirs/latest/dirs/).

| Platform | Value | Example |
| -------- | ----- | ------- |
| Linux | `$XDG_CONFIG_HOME` or `$HOME`/.config | `/home/alice/.config` |
| macOS | `$HOME`/Library/Application Support | `/Users/Alice/Library/Application Support` |
| Windows | `{FOLDERID_RoamingAppData}` | `C:\Users\Alice\AppData\Roaming` |

The `config.toml` is created under the `rust-autoclicker` directory.

Example: `/home/alice/.config/rust-autoclicker/`

### Configuring input device
In order for the keybind (hotkey) to work correctly, you need to set the correct device in the config.toml.
To find out which device is correct, install `evtest` or a different tool capable of monitoring input events. For this guide, we will be using `evtest`.

Then run:

```bash
sudo evtest
```

Look at the list of devices, find which event is your desired device (keyboard / mouse) and set the input_device_path event number to that number.
For instance, if our desired event = 0, we change our input_device_path from "/dev/input/event3" to "/dev/input/event0". 
This should not be done in app runtime to avoid conflicts.

## CLI
Commands:

```bash
config help

config set <VARIABLE> <VALUE> 

config list
```
config help - Displays information about the available commands, built-in with [clap](https://docs.rs/clap/latest/clap/).

config set  - Set a value in config.toml to the given value. Then saves the config.

`<VARIABLE>` is the configuration variable to change, and `<VALUE>` is the string value to assign to it.

For instance:
```bash
./autoclicker config set min_delay 85
```
sets the min_delay in config.toml to 85.

config list - Loads config from config.toml and prints (AppConfig) output.

## How does it work?

Wayland does not allow applications to click in other windows. To bypass this, it uses evdev to create a virtual device that behaves like a mouse. 
The autoclicker sends click events through this virtual device, using VirtualDevice.emit().

## Contributing
All contributions are greatly appreciated. Whether you want to fix a bug, improve existing functionality, add a new feature, or something else, feel free to contribute.

You can contribute by opening an issue, suggesting an idea, submitting a pull request, or you can personally dm me your contribution.