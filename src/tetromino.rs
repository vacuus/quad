use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::matrix::{Matrix, MatrixPosition};
use crate::BLOCK_SIZE;

// A block can be part of the current tetromino
#[derive(Debug)]
pub struct Tetromino;

impl Tetromino {
    fn blocks_from_type(tetromino_type: TetrominoType)
    -> (i16, Color, [(i16, i16); 4]) {
        use self::TetrominoType::*;
    
        let matrix_size = match tetromino_type {
            I | O => 4,
            T | Z | S | L | J => 3,
        };
    
        let color = match tetromino_type {
            I => (0.0, 0.7, 0.7),  // cyan
            O => (0.7, 0.7, 0.0),  // yellow
            T => (0.7, 0.0, 0.7),  // purple
            Z => (0.7, 0.0, 0.0),  // red
            S => (0.0, 0.7, 0.0),  // green
            L => (0.0, 0.0, 0.9),  // blue
            J => (0.9, 0.25, 0.0), // orange
        };

        let color = Color::rgb(color.0, color.1, color.2);

        let positions = match tetromino_type {
            I => [(1, 3), (1, 2), (1, 1), (1, 0)],
            O => [(1, 1), (1, 2), (2, 1), (2, 2)],
            T => [(0, 1), (1, 1), (2, 1), (1, 2)],
            Z => [(0, 2), (1, 2), (1, 1), (2, 1)],
            S => [(2, 2), (1, 2), (1, 1), (0, 1)],
            L => [(0, 2), (0, 1), (1, 1), (2, 1)],
            J => [(0, 1), (1, 1), (2, 1), (2, 2)],
        };

        (matrix_size, color, positions)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}

// Used in pseudorandom generation of tetromino type during spawning
impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        use self::TetrominoType::*;

        match rng.gen_range(0..7) {
            0 => I,
            1 => O,
            2 => T,
            3 => S,
            4 => Z,
            5 => L,
            6 => J,
            _ => unreachable!(),
        }
    }
}

pub fn spawn_tetromino(
    commands: &mut Commands,
    matrix: &Matrix,
    materials: &mut Assets<ColorMaterial>,
    tetromino_type: &mut TetrominoType,
) {
    *tetromino_type = rand::random::<TetrominoType>();

    let (tetromino_matrix_size, color, positions) = Tetromino::blocks_from_type(
        *tetromino_type
    );

    for (x, y) in positions {
        let x = x + 3;
        let y = matrix.height - tetromino_matrix_size + y;

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                sprite: Sprite::new(Vec2::splat(BLOCK_SIZE)),
                transform: Transform::from_translation(
                    Vec3::new(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE, 1.0)
                ),
                ..Default::default()
            })
            .insert(MatrixPosition {
                x,
                y,
            })
            .insert(Tetromino)
        ;
    }
}
