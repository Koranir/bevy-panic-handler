//! A wrapper for panics using Bevy's plugin system.
//!
//! On supported platforms (windows, macos, linux) will produce a popup using the `msgbox` crate in addition to writing via `log::error!`, or if `bevy::log::LogPlugin` is not enabled, `stderr`.

use std::sync::Arc;

use bevy::prelude::*;

pub trait PanicHandleFn: Fn(&std::panic::PanicInfo) + Send + Sync {}
impl<T: Fn(&std::panic::PanicInfo) + Send + Sync> PanicHandleFn for T {}

#[derive(Default)]
pub struct PanicHandler {
    custom_hook: Option<Arc<dyn PanicHandleFn>>,
}
impl PanicHandler {
    #[must_use]
    pub fn new(panic_handler: impl PanicHandleFn + 'static) -> Self {
        Self {
            custom_hook: Some(Arc::new(panic_handler)),
        }
    }

    #[must_use]
    pub fn default_take_panic() -> Self {
        Self {
            custom_hook: Some(Arc::new(std::panic::take_hook()))
        }
    }
}

impl Plugin for PanicHandler {
    fn build(&self, _: &mut App) {
        let custom_hook = self
            .custom_hook
            .as_ref().cloned()
            .unwrap_or_else(|| Arc::new(|_| {}));
        std::panic::set_hook(Box::new(move |info| {
            let info_string = format!(
                "Unhandled panic! @ {}:\n{}",
                info.location()
                    .map_or("Unknown Location".to_owned(), ToString::to_string),
                info.payload().downcast_ref::<String>().unwrap_or(
                    &((*info.payload().downcast_ref::<&str>().unwrap_or(&"No Info")).to_string())
                )
            );

            // Known limitations: Logging in tests prints to stdout immediately.
            // This will print duplicate messages to stdout if the default panic hook is being used & env_logger is initialized.
            bevy::log::error!("{info_string}");

            // Don't interrupt test execution with a popup, and dont try on unsupported platforms.
            #[cfg(all(
                not(test),
                any(target_os = "windows", target_os = "macos", target_os = "linux")
            ))]
            { _ = msgbox::create("Fatal Error", &info_string, msgbox::IconType::Error); }

            custom_hook(info);
        }));
    }
}
