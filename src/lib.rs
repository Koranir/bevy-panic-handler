use bevy::prelude::*;

pub struct PanicPopup;

fn popup_panic_hook(info: &std::panic::PanicInfo) {
    let info = info.to_string();
    bevy::log::error!("Unhandled Panic: {}", info);
    #[cfg(all(not(test), any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    { _ = msgbox::create("Unhandled Panic", &info, msgbox::IconType::Error); }
}

impl Plugin for PanicPopup {
    fn build(&self, _: &mut App) {
        std::panic::set_hook(Box::new(popup_panic_hook));
    }
}
