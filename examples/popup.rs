use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_panic_handler::PanicHandler::default_take_panic())
        .add_systems(Startup, || {
            panic!("Example Error.\nNewlines AOK. 😃\n{} too.", "fmt strings")
        })
        .run();
}
