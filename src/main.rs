use bevy::prelude::*;
mod tilemap;
mod tetramino;
use tilemap::TilemapPlugin;
use tetramino::TetraminoPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Tetris".to_string(),
        width: 800.0,
        height: 600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(TilemapPlugin)
    .add_plugin(TetraminoPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
