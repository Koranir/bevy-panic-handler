# Bevy Panic Handler

[![Crates.io](https://img.shields.io/crates/v/bevy-panic-handler)](https://crates.io/crates/bevy-panic-handler)

A Plugin for [`bevy`](https://github.com/bevyengine/bevy) that creates a popup and logs to error on panic.

## Usage

```rs
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(bevy::log::LogPlugin)
        .add_plugins(bevy_panic_handler::PanicHandler::new().build())
        // Normal bevy code...
}
```

Check examples for more usages.

## Example

[![popup image](./images/Popup.png 'popup.rs')](./examples/popup.rs)

## Version Compatibility

| Bevy | `bevy-panic-handler` |
| ---- | -------------------- |
| 0.18 |                7.0.0 |
| 0.17 |                6.0.0 |
| 0.16 |                5.0.0 |
| 0.15 |                4.0.0 |
| 0.14 |                3.0.0 |
