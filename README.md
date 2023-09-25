# Bevy Panic Handler

[![Crates.io](https://img.shields.io/crates/v/bevy-panic-handler)](https://crates.io/crates/bevy-panic-handler)

A Plugin for [`bevy`](https://github.com/bevyengine/bevy) that creates a popup and logs to error on panic.

## Usage

```rs
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(bevy::log::LogPlugin)
        .add_plugins(bevy_panic_handler::PanicHandler::default())
        // Normal usage...
}
```

`PanicHandler::default()` will only create a popup and log to error, removing any previous panic handler.

`PanicHandler::new(..)` takes a function or static closure to call once the popup is closed.

`PanicHandler::default_take_panic()` will take the already-existing panic handler and call that once the popup is closed. (If the default panic handler is used and the logger is initialized, the panic contents will be written twice to stdout).

## Examples

[![popup image](./images/Popup.png 'popup.rs')](./examples/popup.rs)
