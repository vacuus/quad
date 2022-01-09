use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::matrix::{Matrix, MatrixPosition};
use crate::BLOCK_SIZE;


// A block can be part of the current tetromino
#[derive(Debug, Component)]
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
            I => Color::rgb(0.0, 0.7, 0.7),  // cyan
            O => Color::rgb(0.7, 0.7, 0.0),  // yellow
            T => Color::rgb(0.7, 0.0, 0.7),  // purple
            Z => Color::rgb(0.7, 0.0, 0.0),  // red
            S => Color::rgb(0.0, 0.7, 0.0),  // green
            L => Color::rgb(0.0, 0.0, 0.9),  // blue
            J => Color::rgb(0.9, 0.2, 0.0), // orange
        };

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
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                    color,
                    ..Default::default()
                },
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
