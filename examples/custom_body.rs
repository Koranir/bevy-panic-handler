use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            bevy_panic_handler::PanicHandler::new()
                .set_body_func(|info| {
                    format!(
                        "Panicked at Line {}, Column {}.\nMessage:\n{}",
                        info.location().unwrap().line(),
                        info.location().unwrap().column(),
                        info.payload()
                            .downcast_ref::<String>()
                            .cloned()
                            .unwrap_or_else(|| info
                                .payload()
                                .downcast_ref::<&str>()
                                .unwrap_or(&"")
                                .to_string())
                    )
                })
                .build(),
        )
        .add_systems(Startup, || {
            panic!("Example Message")
        })
        .run();
}
