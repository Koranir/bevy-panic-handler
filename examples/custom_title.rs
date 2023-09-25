use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            bevy_panic_handler::PanicHandler::new()
                .set_title_func(|info| {
                    format!(
                        "Panic at L{}:C{}",
                        info.location().unwrap().line(),
                        info.location().unwrap().column()
                    )
                })
                .build(),
        )
        .add_systems(Startup, || {
            panic!("Example Message")
        })
        .run();
}
