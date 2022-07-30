use bevy::prelude::*;
use crate::tilemap::*;
use rand::Rng;

const I_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
];

const S_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 1, 0],
    [1, 1, 0, 0],
    [0, 0, 0, 0],
];

const Z_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 1, 0, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

const J_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
];

const L_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [1, 0, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
];

const O_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [0, 0, 0, 0],
];

const T_TETRAMINO: [[usize; 4]; 4] = [
    [0, 0, 0, 0],
    [0, 1, 0, 0],
    [1, 1, 1, 0],
    [0, 0, 0, 0],
];

enum TetraminoType {
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
        app.add_startup_system(create_random_tetramino);
    }
}


#[derive(Component)]
pub struct Tetramino {
    pub x: usize,
    pub y: usize,
    pub shape: [[usize; 4]; 4],
}

impl Tetramino {
    pub fn new() -> Self {
        Self { 
            x: COLS / 2 - 2, 
            y: ROWS - 4, 
            shape: [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ]
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
    }
}

pub fn create_random_tetramino(mut commands: Commands) {
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
                            transform: Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, (3 - y) as f32 * TILE_SIZE, 0.0)),
                            ..Default::default()
                        });
                    }
                }
            }
        })
        .insert(tetramino);

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
}

#[cfg(test)]
#[test]
fn test_create_tetramino() {
    let mut tetramino = Tetramino::new();
    tetramino.set_shape(&TetraminoType::I);
    assert_eq!(tetramino.shape, I_TETRAMINO);
    assert_eq!(tetramino.x, COLS / 2 - 2);
    assert_eq!(tetramino.y, ROWS - 4);
}