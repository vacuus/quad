use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::matrix::{Matrix, MatrixPosition};
use crate::BLOCK_SIZE;


// starting positions
pub const I: [(i16, i16); 4]  = [(0, 1), (1, 1), (2, 1), (3, 1)];
pub const I_ORIGIN: MatrixPosition = MatrixPosition { x: 2, y: 1};
pub const I_COLOR: Color = Color::rgb(0.0, 0.7, 0.7); // cyan

pub const O: [(i16, i16); 4]  = [(1, 1), (1, 2), (2, 1), (2, 2)];
pub const O_ORIGIN: MatrixPosition = MatrixPosition { x: 2, y: 2};
pub const O_COLOR: Color = Color::rgb(0.7, 0.7, 0.0); // yellow

pub const T: [(i16, i16); 4]  = [(0, 1), (1, 1), (2, 1), (1, 2)];
pub const T_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 1};
pub const T_COLOR: Color = Color::rgb(0.7, 0.0, 0.7); // purple

pub const Z: [(i16, i16); 4]  = [(0, 1), (1, 1), (1, 0), (2, 0)];
pub const Z_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const Z_COLOR: Color = Color::rgb(0.7, 0.0, 0.0); // red

pub const S: [(i16, i16); 4]  = [(2, 1), (1, 1), (1, 0), (0, 0)];
pub const S_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const S_COLOR: Color = Color::rgb(0.0, 0.7, 0.0); // green

pub const L: [(i16, i16); 4]  = [(0, 0), (1, 0), (2, 0), (2, 1)];
pub const L_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const L_COLOR: Color = Color::rgb(0.0, 0.0, 0.9); // blue

pub const J: [(i16, i16); 4]  = [(0, 0), (0, 1), (1, 0), (2, 0)];
pub const J_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const J_COLOR: Color = Color::rgb(0.9, 0.2, 0.0); // orange


// denotes a block that is part of the current tetromino
#[derive(Debug, Component)]
pub struct TetrominoBlock;


#[derive(Copy, Clone, Debug)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
//     unimplemented
//     Other(u16),
}

// used in pseudorandom generation of tetromino type during spawning
impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        use self::TetrominoType::*;

//         match rng.gen_range(..) {
        match rng.gen_range(0..7) {
            0 => I,
            1 => O,
            2 => T,
            3 => S,
            4 => Z,
            5 => L,
            6 => J,
            _ => unreachable!(),
//             unimplemented
//             n => Other(n),
        }
    }
}


pub fn spawn_tetromino(
    commands: &mut Commands,
    matrix: &Matrix,
    origin: &mut MatrixPosition,
) {
    let tetromino_type = rand::random::<TetrominoType>();
    let (positions, rotation_origin, color) = match tetromino_type {
        TetrominoType::I => (I, I_ORIGIN, I_COLOR),
        TetrominoType::O => (O, O_ORIGIN, O_COLOR),
        TetrominoType::T => (T, T_ORIGIN, T_COLOR),
        TetrominoType::S => (S, S_ORIGIN, S_COLOR),
        TetrominoType::Z => (Z, Z_ORIGIN, Z_COLOR),
        TetrominoType::L => (L, L_ORIGIN, L_COLOR),
        TetrominoType::J => (J, J_ORIGIN, J_COLOR),
//         unimplemented
//         _ => ???,
    };

    *origin = rotation_origin;

    for (x, y) in positions {
        let x = x + 3;
        // fix
        let y = matrix.height - 4 + y;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                    color,
                    ..Sprite::default()
                },
                transform: Transform::from_translation(
                    Vec3::new(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE, 1.0)
                ),
                ..SpriteBundle::default()
            })
            .insert(MatrixPosition { x, y })
            .insert(TetrominoBlock)
        ;
    }
}
