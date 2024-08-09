use bevy::prelude::*;

mod systems;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
    ));

    app.add_systems(Startup, setup);
    app.add_systems(Update, systems::mouse_click_system::mouse_click_system);
    app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ships/saucer.png"),
        ..default()
    });
}
