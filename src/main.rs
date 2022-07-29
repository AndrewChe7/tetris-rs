use bevy::prelude::*;

const ROWS: usize = 20;
const COLS: usize = 10;
const TILE_SIZE: f32 = 20.0;

#[derive(Component)]
struct Tile {
    x: usize,
    y: usize,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Tetris".to_string(),
        width: 800.0,
        height: 600.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature="debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup)
        .add_startup_system(tiles_setup)
        .run();
    
    

}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn tiles_setup(mut commands: Commands) {
    for x in 0..COLS {
        for y in 0..ROWS {
            let tile_position = Vec3::new(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                0.0,
            );

            let tile_position = tile_position - Vec3::new(
                TILE_SIZE * COLS as f32 / 2.0, 
                TILE_SIZE * ROWS as f32 / 2.0, 
                0.0
            );

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(tile_position),
                ..Default::default()
            })
            .insert(Tile { x, y });
        }
    }
}