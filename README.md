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

## Example

[![popup image](./images/Popup.png 'popup.rs')](./examples/popup.rs)
