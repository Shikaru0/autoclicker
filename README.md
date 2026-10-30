# Autoclicker

A lightweight UNIX autoclicker written in Rust with Wayland support, by using [evdev](https://docs.rs/evdev/latest/evdev/) for input handling.

[![CI](https://github.com/Shikaru0/autoclicker/actions/workflows/rust.yml/badge.svg)](https://github.com/Shikaru0/autoclicker/actions/workflows/rust.yml)
[![GitHub Release](https://img.shields.io/github/v/release/Shikaru0/autoclicker)](https://github.com/Shikaru0/autoclicker/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-CE412B?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Wayland](https://img.shields.io/badge/Wayland-Supported-FFCC00?logo=wayland&logoColor=black)](https://wayland.freedesktop.org/)

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

If you ran the application atleast once (causing it to generate a default `config.toml`), you can also run `config path` to see the (absolute) path to `config.toml`.

### Configuring input device
In order for the keybind (hotkey) to work correctly, you need to set the correct device in the config.toml.
To find out which device is correct, install `evtest` or a different tool capable of monitoring input events. For this guide, we will be using `evtest`.

Then run:

```bash
sudo evtest
```

Look at the list of devices, find which event is your desired device (keyboard / mouse) and set the input_device_path event number to that number.
For instance, if our desired event = 0, we change our input_device_path from "/dev/input/event3" to "/dev/input/event0". 
This can also be changed at runtime — the input thread reopens the new device automatically (see [How does application config reload?](#how-does-application-config-reload)).

### How does application config reload?
While the autoclicker is running, it watches `config.toml` for changes using [notify](https://docs.rs/notify/latest/notify/).
This means `config set ...` from another terminal, or a direct file edit, takes effect without needing to restart.

## CLI
Commands:

```bash
config help
config set <VARIABLE> <VALUE>
config get <VARIABLE>
config list
config path
```

- `config help` — Displays information about the available commands, built-in with [clap](https://docs.rs/clap/latest/clap/).
- `config set <VARIABLE> <VALUE>` — Set a value in config.toml to the given value. Then saves the config.
- `config get <VARIABLE>` — Get the value of a configuration variable from config.toml.
- `config list` — Loads config from config.toml and prints (AppConfig) output.
- `config path` — Prints the absolute path to config.toml.

`<VARIABLE>` is the configuration variable to change (for `set`) or read (for `get`), and `<VALUE>` is the string value to assign to it.

Available variables:

| Variable | Type | Description |
| -------- | ---- | ----------- |
| `min_delay` | integer (ms) | Minimum delay between bursts. |
| `max_delay` | integer (ms) | Maximum delay between bursts. A random value between `min_delay` and `max_delay` is used per cycle. |
| `min_burst_delay` | integer (ms) | Minimum delay between clicks inside a burst. |
| `max_burst_delay` | integer (ms) | Maximum delay between clicks inside a burst. A random value between `min_burst_delay` and `max_burst_delay` is used between consecutive clicks. |
| `click_count` | integer (≥1) | Number of clicks per burst. `1` is a single click, `2` a double-click, `3` a triple-click, and so on. |
| `keybind` | string | evdev key code for the hotkey, e.g. `KEY_F1`. |
| `key` | `left` \| `right` `extra` \| `middle` `side` \| `forward` \| `back` | Which mouse button to click. |
| `toggle` | boolean | If `true`, the hotkey toggles clicking on/off. If `false`, clicking happens only while the hotkey is held. |
| `input_device_path` | string | Path to the evdev device to read the hotkey from, e.g. `/dev/input/event3`. |

For instance:
```bash
./autoclicker config set min_delay 85
```
sets the min_delay in config.toml to 85.

```bash
./autoclicker config set click_count 2
./autoclicker config set min_burst_delay 10
./autoclicker config set max_burst_delay 30
```
configures a double-click burst with 10–30ms between the two clicks.

## How does it work?

Wayland does not allow applications to click in other windows. To bypass this, it uses evdev to create a virtual device that behaves like a mouse. 
The autoclicker sends click events through this virtual device, using VirtualDevice.emit().

## Contributing
All contributions are greatly appreciated. Whether you want to fix a bug, improve existing functionality, add a new feature, or something else, feel free to contribute.

You can contribute by opening an issue, suggesting an idea, submitting a pull request, or you can personally dm me your contribution.