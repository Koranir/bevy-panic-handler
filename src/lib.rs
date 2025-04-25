//! A wrapper for panics using Bevy's plugin system.
//!
//! On supported platforms (windows, macos, linux) will produce a popup using the `msgbox` crate in addition to logging through `bevy_log` if the `log` feature is enabled.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use std::sync::Arc;

use bevy::prelude::*;

pub trait PanicHandleFn<Res>:
    Fn(&std::panic::PanicHookInfo) -> Res + Send + Sync + 'static
{
}
impl<Res, T: Fn(&std::panic::PanicHookInfo) -> Res + Send + Sync + 'static> PanicHandleFn<Res>
    for T
{
}

#[derive(Default)]
pub struct PanicHandlerBuilder {
    name: Option<Arc<dyn PanicHandleFn<String>>>,
    body: Option<Arc<dyn PanicHandleFn<String>>>,
    hook: Option<Arc<dyn PanicHandleFn<()>>>,
}
impl PanicHandlerBuilder {
    #[must_use]
    /// Builds the `PanicHandler`
    pub fn build(self) -> PanicHandler {
        PanicHandler {
            title: {
                self.name.unwrap_or_else(|| {
                    Arc::new(|_: &std::panic::PanicHookInfo| "Fatal Error".to_owned())
                })
            },
            body: {
                self.body.unwrap_or_else(|| {
                    Arc::new(|info| {
                        format!(
                            "Unhandled panic! at {}:\n{}",
                            info.location()
                                .map_or("Unknown Location".to_owned(), ToString::to_string),
                            info.payload().downcast_ref::<String>().map_or_else(
                                || (*info.payload().downcast_ref::<&str>().unwrap_or(&"No Info"))
                                    .to_string(),
                                ToOwned::to_owned,
                            )
                        )
                    })
                })
            },
            hook: { self.hook.unwrap_or_else(|| Arc::new(|_| {})) },
        }
    }

    #[must_use]
    /// After the popup is closed, the previously existing panic hook will be called
    pub fn take_call_from_existing(mut self) -> Self {
        self.hook = Some(Arc::new(std::panic::take_hook()));
        self
    }

    #[must_use]
    /// After the popup is closed, this function will be called
    pub fn set_call_func(mut self, call_func: impl PanicHandleFn<()>) -> Self {
        self.hook = Some(Arc::new(call_func));
        self
    }

    #[must_use]
    /// The popup title will be set to the result of this function
    pub fn set_title_func(mut self, title_func: impl PanicHandleFn<String>) -> Self {
        self.name = Some(Arc::new(title_func));
        self
    }

    #[must_use]
    /// The popup body will be set to the result of this function
    pub fn set_body_func(mut self, body_func: impl PanicHandleFn<String>) -> Self {
        self.body = Some(Arc::new(body_func));
        self
    }
}

/// Bevy plugin that opens a popup window on panic & logs an error
#[derive(Clone)]
pub struct PanicHandler {
    pub title: Arc<dyn PanicHandleFn<String>>,
    pub body: Arc<dyn PanicHandleFn<String>>,
    pub hook: Arc<dyn PanicHandleFn<()>>,
}
impl PanicHandler {
    #[must_use]
    #[allow(clippy::new_ret_no_self)]
    /// Create a new builder. The custom hook does nothing.
    pub fn new() -> PanicHandlerBuilder {
        PanicHandlerBuilder::default()
    }

    #[must_use]
    /// Create a new builder. The custom hook is taken from `std::panic::take_hook()`
    pub fn new_take_old() -> PanicHandlerBuilder {
        PanicHandlerBuilder::default().take_call_from_existing()
    }
}

impl Plugin for PanicHandler {
    fn build(&self, _: &mut App) {
        let handler = self.clone();
        std::panic::set_hook(Box::new(move |info| {
            #[cfg(not(test))]
            let title_string = (handler.title)(info);
            #[cfg(not(test))]
            let info_string = (handler.body)(info);

            // This will print duplicate messages to stdout if the default panic hook is being used & env_logger is initialized.
            #[cfg(all(not(test), feature = "log"))]
            bevy::log::error!("{title_string}\n{info_string}");

            // Don't interrupt test execution with a popup, and dont try on unsupported platforms.
            #[cfg(all(
                not(test),
                any(target_os = "windows", target_os = "macos", target_family = "unix")
            ))]
            {
                let builder = native_dialog::MessageDialogBuilder::default()
                    .set_title(&title_string)
                    .set_text(&info_string)
                    .set_level(native_dialog::MessageLevel::Error);
                if let Err(e) = builder.alert().show() {
                    #[cfg(feature = "log")]
                    bevy::log::error!("{e}");
                    #[cfg(not(feature = "log"))]
                    {
                        _ = e;
                    }
                }
            }

            (handler.hook)(info);
        }));
    }
}
