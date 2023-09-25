use bevy::prelude::*;

fn main() {
    // The path we used to run the executable
    let running_name = std::env::args().next().unwrap();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            bevy_panic_handler::PanicHandler::new()
                .set_call_func(move |_| {
                    println!("Panicked! Arg 1 was: {running_name}");
                })
                .set_body_func(|_| {
                    let mut res_str = String::new();
                    std::io::stdin().read_line(&mut res_str).unwrap();
                    format!("Panicked, also got me a message: {res_str}")
                })
                .build(),
        )
        .add_systems(Startup, || panic!("Example Error. Closures work fine too."))
        .run();
}
