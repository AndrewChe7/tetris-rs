use bevy::prelude::*;

pub const ROWS: usize = 20;
pub const COLS: usize = 10;
pub const TILE_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub value: u8,
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(tiles_setup);
    }
}

pub fn get_coordinate(x: &usize, y: &usize) -> Vec3 {
    let mut res = Vec3::new(
        *x as f32 * TILE_SIZE, 
        *y as f32 * TILE_SIZE,
        0.0
    );
    res -= Vec3::new(
        TILE_SIZE * COLS as f32 / 2.0, 
        TILE_SIZE * ROWS as f32 / 2.0, 
        0.0
    );
    res
}

pub fn tiles_setup(mut commands: Commands) {
    for x in 0..COLS {
        for y in 0..ROWS {
            let tile_position = get_coordinate(&x, &y);

            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(tile_position),
                ..Default::default()
            })
            .insert(Tile { x, y, value: 0 });
        }
    }
}

#[cfg(test)]
#[test]
fn test_get_coordinate() {
    assert_eq!(get_coordinate(&0, &0), Vec3::new(
        -TILE_SIZE * COLS as f32 / 2.0, 
        -TILE_SIZE * ROWS as f32 / 2.0, 
        0.0
    ));
    assert_eq!(get_coordinate(&1, &0), Vec3::new(
        -TILE_SIZE * COLS as f32 / 2.0 + TILE_SIZE, 
        -TILE_SIZE * ROWS as f32 / 2.0, 
        0.0
    ));
    assert_eq!(get_coordinate(&0, &1), Vec3::new(
        -TILE_SIZE * COLS as f32 / 2.0, 
        -TILE_SIZE * ROWS as f32 / 2.0 + TILE_SIZE, 
        0.0
    ));
    assert_eq!(get_coordinate(&0, &10), Vec3::new(
        -TILE_SIZE * COLS as f32 / 2.0, 
        -TILE_SIZE * ROWS as f32 / 2.0 + TILE_SIZE * 10.0, 
        0.0
    ));
}