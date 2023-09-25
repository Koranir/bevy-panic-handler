use bevy::prelude::*;

fn main() {
    // The path we used to run the executable
    let running_name = std::env::args().next().unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_panic_handler::PanicHandler::new(move |_| {
            println!("Panicked! Arg 1 was: {running_name}");
        }))
        .add_systems(Startup, || {
            panic!("Example Error.\nNewlines AOK. 😃\n{} too.", "fmt strings")
        })
        .run();
}

// Result:

// (omitted)
// 2023-09-25T23:00:21.128651Z ERROR bevy_panic_handler: Unhandled panic! @ examples/closure.rs:12:13:
// Example Error.
// Newlines AOK. 😃
// fmt strings too.
// (omitted)
// Panicked! Arg 1 was: target/debug/examples/closure
// Encountered a panic in system `closure::main::{{closure}}`!
// Encountered a panic in system `bevy_app::main_schedule::Main::run_main`!
