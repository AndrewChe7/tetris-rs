use bevy::{prelude::*, core::FixedTimestep};
use crate::tilemap::*;
use rand::Rng;

const I_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

const S_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 1, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
];

const Z_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

const J_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
];

const L_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 1, 0],
    [1, 0, 0, 0],
    [0, 0, 0, 0],
];

const O_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

const T_TETRAMINO: [[u8; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
];

#[derive(Clone, Copy)]
pub enum TetraminoType {
    I,
    S,
    Z,
    J,
    L,
    O,
    T,
}

pub struct TetraminoPlugin;

impl Plugin for TetraminoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_random_tetramino_system)
            .add_event::<CollidedEvent>()
            .add_system(on_tetramino_changed)
            .add_system(keyboard_input);
        
        app.add_stage("Tetramino fall", 
            SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(0.5).with_label("fall"))
                    .with_system(fall_system)
        );

    }
}


#[derive(Component, Clone, Copy)]
pub struct Tetramino {
    pub x: i32,
    pub y: i32,
    pub shape: [[u8; 4]; 4],
    pub tetramino_type: TetraminoType,
}

struct CollidedEvent;

impl Tetramino {
    pub fn new() -> Self {
        Self { 
            x: COLS as i32 / 2 - 2, 
            y: ROWS as i32 - 4, 
            shape: [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            tetramino_type: TetraminoType::I,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        let mut new_shape = [[0; 4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                new_shape[y][3 - x] = self.shape[x][y];
            }
        }
        self.shape = new_shape;
    }

    pub fn rotate_conterclockwise(&mut self) {
        let mut new_shape = [[0; 4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                new_shape[3 - y][x] = self.shape[x][y];
            }
        }
        self.shape = new_shape;
    }

    pub fn get_bounds(&self) -> (i32, i32, i32, i32) {
        let mut min_x: i32 = 3;
        let mut min_y: i32 = 3;
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;
        for x in 0..4 {
            for y in 0..4 {
                if self.shape[x as usize][y as usize] == 1 {
                    if x < min_x {
                        min_x = x;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
        (min_x, min_y, max_x, max_y)
    }

    pub fn set_shape(&mut self, tetramino_type: &TetraminoType) {
        match tetramino_type {
            TetraminoType::I => self.shape = I_TETRAMINO,
            TetraminoType::S => self.shape = S_TETRAMINO,
            TetraminoType::Z => self.shape = Z_TETRAMINO,
            TetraminoType::J => self.shape = J_TETRAMINO,
            TetraminoType::L => self.shape = L_TETRAMINO,
            TetraminoType::O => self.shape = O_TETRAMINO,
            TetraminoType::T => self.shape = T_TETRAMINO,
        }
        self.tetramino_type = *tetramino_type;
    }
}

pub fn create_random_tetramino_system(mut commands: Commands) {
    create_random_tetramino(&mut commands);
}

pub fn create_random_tetramino(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let tetramino_type = match rng.gen_range(0..7) {
        0 => TetraminoType::I,
        1 => TetraminoType::S,
        2 => TetraminoType::Z,
        3 => TetraminoType::J,
        4 => TetraminoType::L,
        5 => TetraminoType::O,
        6 => TetraminoType::T,
        _ => unreachable!(),
    };

    let mut tetramino = Tetramino::new();
    tetramino.set_shape(&tetramino_type);

    let color = match &tetramino_type {
        TetraminoType::I => Color::rgb(0.0, 0.0, 1.0),
        TetraminoType::S => Color::rgb(1.0, 0.0, 0.0),
        TetraminoType::Z => Color::rgb(0.0, 1.0, 0.0),
        TetraminoType::J => Color::rgb(0.0, 1.0, 1.0),
        TetraminoType::L => Color::rgb(1.0, 1.0, 0.0),
        TetraminoType::O => Color::rgb(1.0, 0.5, 0.0),
        TetraminoType::T => Color::rgb(0.5, 0.0, 1.0),
    };

    commands.spawn_bundle(
        TransformBundle::from_transform(Transform {
            translation: get_coordinate(&tetramino.x, &tetramino.y) + Vec3::new(0.0, 0.0, 0.1),
            ..default()
        })
        )
        .with_children(|parent| {
            for x in 0..4 {
                for y in 0..4 {
                    if tetramino.shape[x][y] == 1 {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0)),
                            ..Default::default()
                        });
                    }
                }
            }
        })
        .insert(tetramino);

}

fn on_tetramino_changed(mut commands: Commands, 
                        q_children: Query<(Entity, &Parent, &Sprite)>,
                        mut q_parent: Query<(Entity, &Tetramino, &mut Transform), Changed<Tetramino>>) {
    for (tetramino_entity, tetramino, mut transform) in q_parent.iter_mut() {
        for (entity, parent, _sprite) in q_children.iter() {
            if parent.0 != tetramino_entity {
                continue;
            }
            commands.entity(entity).despawn();
        }

        transform.translation = get_coordinate(&tetramino.x, &tetramino.y) + Vec3::new(0.0, 0.0, 0.1);

        let color = match &tetramino.tetramino_type {
            TetraminoType::I => Color::rgb(0.0, 0.0, 1.0),
            TetraminoType::S => Color::rgb(1.0, 0.0, 0.0),
            TetraminoType::Z => Color::rgb(0.0, 1.0, 0.0),
            TetraminoType::J => Color::rgb(0.0, 1.0, 1.0),
            TetraminoType::L => Color::rgb(1.0, 1.0, 0.0),
            TetraminoType::O => Color::rgb(1.0, 0.5, 0.0),
            TetraminoType::T => Color::rgb(0.5, 0.0, 1.0),
        };

        commands.entity(tetramino_entity).with_children(|parent| {
            for x in 0..4 {
                for y in 0..4 {
                    if tetramino.shape[x][y] == 1 {
                        parent.spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0)),
                            ..Default::default()
                        });
                    }
                }
            }
        });
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Tetramino)>,
    tiles_query: Query<(Entity, &mut Tile)>,
) {
    let is_collided_left = check_is_collided(&query,&tiles_query, (-1, 0), 0);
    let is_collided_right = check_is_collided(&query,&tiles_query, (1, 0), 0);
    let is_collided_clockwise = check_is_collided(&query,&tiles_query, (0, 0), 1);
    let is_collided_conterclockwise = check_is_collided(&query,&tiles_query, (0, 0), -1);
    if keys.just_pressed(KeyCode::D) {
        for (_, mut tetramino) in query.iter_mut() {
            if !is_collided_conterclockwise {
                tetramino.rotate_conterclockwise();
            }
        }
    } else if keys.just_pressed(KeyCode::F) {
        for (_, mut tetramino) in query.iter_mut() {
            if !is_collided_clockwise {
                tetramino.rotate_clockwise();
            }
        }
    }
    if keys.just_pressed(KeyCode::J) {
        for (_, mut tetramino) in query.iter_mut() {
            let (min_x, _, _, _) = tetramino.get_bounds();
            if tetramino.x > -min_x && !is_collided_left {
                tetramino.x -= 1;
            }
        }
    } else if keys.just_pressed(KeyCode::K) {
        for (_, mut tetramino) in query.iter_mut() {
            let (_, _, max_x, _) = tetramino.get_bounds();
            if tetramino.x < COLS as i32 - max_x - 1 && !is_collided_right {
                tetramino.x += 1;
            }
        }
    }

}

fn check_is_collided(tetramino_query: &Query<(Entity, &mut Tetramino)>,
                     field_query: &Query<(Entity, &mut Tile)>,
                     delta: (i32, i32),
                     rotation: i32) -> bool {
    for (_, tetramino) in tetramino_query.iter() {
        let (min_x, min_y, max_x, max_y) = tetramino.get_bounds();
        if delta.1 == -1 && tetramino.y + min_y == 0 {
            return true;
        }
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                if tetramino.shape[x as usize][y as usize] == 1 {
                    match rotation {
                        0 => {
                            let tile_x = x + tetramino.x + delta.0;
                            let tile_y = y + tetramino.y + delta.1;
                            for (_entity, tile) in field_query.iter() {
                                if tile.x == tile_x && tile.y == tile_y && tile.value > 0 {
                                    return true;
                                }
                            }
                        },
                        -1 => {
                            let mut tetramino_cloned = tetramino.clone();
                            tetramino_cloned.rotate_conterclockwise();
                            let (min_x, min_y, max_x, max_y) = tetramino_cloned.get_bounds();
                            if tetramino_cloned.y + min_y <= 0 {
                                return true;
                            }
                            if tetramino_cloned.y + max_y >= ROWS as i32 - 1 {
                                return true;
                            }
                            if tetramino_cloned.x + min_x <= 0 {
                                return true;
                            }
                            if tetramino_cloned.x + max_x >= COLS as i32 - 1 {
                                return true;
                            }
                            let tile_x = x + tetramino_cloned.x;
                            let tile_y = y + tetramino_cloned.y;
                            for (_entity, tile) in field_query.iter() {
                                if tile.x == tile_x && tile.y == tile_y && tile.value > 0 {
                                    return true;
                                }
                            }
                        },
                        1 => {
                            let mut tetramino_cloned = tetramino.clone();
                            tetramino_cloned.rotate_clockwise();
                            if tetramino_cloned.y + min_y <= 0 {
                                return true;
                            }
                            if tetramino_cloned.y + max_y >= ROWS as i32 - 1 {
                                return true;
                            }
                            if tetramino_cloned.x + min_x <= 0 {
                                return true;
                            }
                            if tetramino_cloned.x + max_x >= COLS as i32 - 1 {
                                return true;
                            }
                            let tile_x = x + tetramino_cloned.x;
                            let tile_y = y + tetramino_cloned.y;
                            for (_entity, tile) in field_query.iter() {
                                if tile.x == tile_x && tile.y == tile_y && tile.value > 0 {
                                    return true;
                                }
                            }
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        }
    }
    return false;
}

fn on_collided( commands: &mut Commands,
                tetramino_query: &Query<(Entity, &mut Tetramino)>,
                field_query: &mut Query<(Entity, &mut Tile)>) {
    for (entity, tetramino) in tetramino_query.iter() {
        for (_tile_entity, mut tile) in field_query.iter_mut() {
            let x = tile.x - tetramino.x;
            if x < 0 || x >= 4 {
                continue;
            }
            let y = tile.y - tetramino.y;
            if y < 0 || y >= 4 {
                continue;
            }
            if tetramino.shape[x as usize][y as usize] == 1 {
                tile.value = match tetramino.tetramino_type {
                    TetraminoType::I => 1,
                    TetraminoType::S => 2,
                    TetraminoType::Z => 3,
                    TetraminoType::J => 4,
                    TetraminoType::L => 5,
                    TetraminoType::O => 6,
                    TetraminoType::T => 7,
                }
            }
        }
        create_random_tetramino(commands);
        commands.entity(entity).despawn_recursive();
    }
}

fn fall_system(mut commands: Commands,
               mut tetramino_query: Query<(Entity, &mut Tetramino)>,
               mut field_query: Query<(Entity, &mut Tile)>) {
    if check_is_collided(&tetramino_query, &field_query, (0, -1), 0) {
        on_collided(&mut commands, &tetramino_query, &mut field_query);
        return;
    }
    for (_entity, mut tetramino) in tetramino_query.iter_mut() {
        tetramino.y -= 1;
    }
}

#[cfg(test)]
#[test]
fn test_set_shape() {
    let mut tetramino = Tetramino::new();
    tetramino.set_shape(&TetraminoType::I);
    assert_eq!(tetramino.shape, I_TETRAMINO);
    tetramino.set_shape(&TetraminoType::S);
    assert_eq!(tetramino.shape, S_TETRAMINO);
    tetramino.set_shape(&TetraminoType::Z);
    assert_eq!(tetramino.shape, Z_TETRAMINO);
    tetramino.set_shape(&TetraminoType::J);
    assert_eq!(tetramino.shape, J_TETRAMINO);
    tetramino.set_shape(&TetraminoType::L);
    assert_eq!(tetramino.shape, L_TETRAMINO);
    tetramino.set_shape(&TetraminoType::O);
    assert_eq!(tetramino.shape, O_TETRAMINO);
    tetramino.set_shape(&TetraminoType::T);
    assert_eq!(tetramino.shape, T_TETRAMINO);
}

#[cfg(test)]
#[test]
fn test_rotate_clockwise() {
    let mut tetramino = Tetramino::new();
    tetramino.set_shape(&TetraminoType::I);
    tetramino.rotate_clockwise();
    assert_eq!(tetramino.shape, [
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 1, 0],
    ]);
    tetramino.set_shape(&TetraminoType::S);
    tetramino.rotate_clockwise();
    assert_eq!(tetramino.shape, [
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
        [0, 0, 0, 0],
    ]);
}

#[cfg(test)]
#[test]
fn test_rotate_conterclockwise() {
    let mut tetramino = Tetramino::new();
    tetramino.set_shape(&TetraminoType::I);
    tetramino.rotate_conterclockwise();
    assert_eq!(tetramino.shape, [
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 0, 0],
    ]);
    tetramino.set_shape(&TetraminoType::S);
    tetramino.rotate_conterclockwise();
    assert_eq!(tetramino.shape, [
        [0, 0, 0, 0],
        [0, 1, 0, 0],
        [0, 1, 1, 0],
        [0, 0, 1, 0],
    ]);
}

#[cfg(test)]
#[test]
fn test_create_tetramino() {
    let mut tetramino = Tetramino::new();
    tetramino.set_shape(&TetraminoType::I);
    assert_eq!(tetramino.shape, I_TETRAMINO);
    assert_eq!(tetramino.x, COLS as i32 / 2 - 2);
    assert_eq!(tetramino.y, ROWS as i32 - 4);
}