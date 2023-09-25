//! A wrapper for panics using Bevy's plugin system.
//!
//! On supported platforms (windows, macos, linux) will produce a popup using the `msgbox` crate in addition to writing via `log::error!`, or if `bevy::log::LogPlugin` is not enabled, `stderr`.

use bevy::prelude::*;

pub struct PanicHandler;

impl Plugin for PanicHandler {
    fn build(&self, app: &mut App) {
        let use_log = app.is_plugin_added::<bevy::log::LogPlugin>();
        std::panic::set_hook(Box::new(move |info| {
            let info = format!(
                "Unhandled panic @ {}:\n{}",
                info.location()
                    .map_or("Unknown Location".to_owned(), ToString::to_string),
                info.payload().downcast_ref::<String>().unwrap_or(
                    &((
                        *info.payload()
                        .downcast_ref::<&str>()
                        .unwrap_or(&"No Info")
                    ).to_string())
                )
            );
            if use_log { error!("{}", info); } else { eprintln!("{info}"); }
            #[cfg(all(not(test), any(target_os = "windows", target_os = "macos", target_os = "linux")))]
            { _ = msgbox::create("Fatal Error", &info, msgbox::IconType::Error); }
        }));
    }
}
