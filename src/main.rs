use bevy::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            fit_canvas_to_parent: true,
            ..Default::default()
        },
        ..Default::default()
    }))
    .insert_resource(ClearColor(Color::hex("dabe8f").unwrap_or_default()));

    app.run();
}
