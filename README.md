# Bevy Panic handler

A Plugin for [`bevy`](https://github.com/bevyengine/bevy) that creates a popup and logs to error on panic.

## Usage

```rs
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(bevy::log::LogPlugin)
        .add_plugins(bevy_panic_handler::PanicHandler)
        // Normal usage...
}
```
