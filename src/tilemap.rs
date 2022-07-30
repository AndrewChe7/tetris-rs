use bevy::prelude::*;

pub const ROWS: usize = 20;
pub const COLS: usize = 10;
pub const TILE_SIZE: f32 = 20.0;

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub value: u8,
}

#[derive(Component)]
struct Score;

#[derive(Default)]
pub struct TetrisData {
    pub score: i32,
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(tiles_setup)
            .init_resource::<TetrisData>()
            .add_startup_system(create_score_text)
            .add_system(on_tile_change)
            .add_system(burn_the_line)
            .add_system(update_score_text);
    }
}

fn create_score_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
    .spawn_bundle(TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: "Score: 0".to_string(),
                style: TextStyle {
                    font: asset_server.load("font.otf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            }],
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.2)),
        ..Default::default()
    }) 
    .insert(Score);
}

fn update_score_text(mut commands: Commands, score: Res<TetrisData>, mut score_text: Query<(&mut Text, &Score)>) {
    let score = score.score;
    for (mut score_text, _) in score_text.iter_mut() {
        score_text.sections[0].value = format!("Score: {}", score);
    }
}

pub fn get_coordinate(x: &i32, y: &i32) -> Vec3 {
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
    for x in 0..COLS as i32 {
        for y in 0..ROWS as i32 {
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

fn on_tile_change(mut query: Query<(Entity, &Tile, &mut Sprite), Changed<Tile>>) {
    for (_entity, tile, mut sprite) in query.iter_mut() {
        sprite.as_mut().color = match tile.value {
            0 => Color::rgb(0.0, 0.0, 0.0), // nothing
            1 => Color::rgb(0.0, 0.0, 1.0), // I
            2 => Color::rgb(1.0, 0.0, 0.0), // S
            3 => Color::rgb(0.0, 1.0, 0.0), // Z
            4 => Color::rgb(0.0, 1.0, 1.0), // J
            5 => Color::rgb(1.0, 1.0, 0.0), // L
            6 => Color::rgb(1.0, 0.5, 0.0), // O
            7 => Color::rgb(0.5, 0.0, 1.0), // T
            _ => unreachable!(),
        };
    }
}

fn burn_the_line(mut tetris_data: ResMut<TetrisData>,
                 mut query: Query<&mut Tile>) {
    let mut matrix = vec![vec![0; COLS]; ROWS + 1];
    for tile in query.iter() {
        matrix[tile.y as usize][tile.x as usize] = tile.value;
    }
    let mut burned = true;
    let mut count = 0;
    let mut line = 0;
    while burned {
        burned = false;
        for y in 0..ROWS {
            let mut line_is_full = true;
            for x in 0..COLS {
                if matrix[y][x] == 0 {
                    line_is_full = false;
                    break;
                }
            }
            if line_is_full {
                burned = true;
                count += 1;
                line = y;
                break;
            }
        }
        if burned {
            for y in line..ROWS {
                for x in 0..COLS {
                    matrix[y][x] = matrix[y + 1][x];
                }
            }
        }
    }
    if count > 0 {
        for mut tile in query.iter_mut() {
            tile.value = matrix[tile.y as usize][tile.x as usize];
        }
        tetris_data.score += count * count * 100;
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