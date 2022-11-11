use bevy::prelude::*;
use rand::Rng;
use crate::matrix::{Matrix, MatrixPosition};
use crate::BLOCK_SIZE;


// starting positions
pub const I: [(i16, i16); 4]  = [(0, 1), (1, 1), (2, 1), (3, 1)];
pub const I_ORIGIN: MatrixPosition = MatrixPosition { x: 2, y: 1};
pub const I_COLOR: Color = Color::rgb(0.0, 0.7, 0.7); // cyan

pub const O: [(i16, i16); 4]  = [(0, 0), (0, 1), (1, 0), (1, 1)];
pub const O_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 1};
pub const O_COLOR: Color = Color::rgb(0.7, 0.7, 0.0); // yellow

pub const T: [(i16, i16); 4]  = [(0, 0), (1, 0), (2, 0), (1, 1)];
pub const T_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const T_COLOR: Color = Color::rgb(0.7, 0.0, 0.7); // purple

pub const Z: [(i16, i16); 4]  = [(0, 1), (1, 1), (1, 0), (2, 0)];
pub const Z_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const Z_COLOR: Color = Color::rgb(0.7, 0.0, 0.0); // red

pub const S: [(i16, i16); 4]  = [(2, 1), (1, 1), (1, 0), (0, 0)];
pub const S_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const S_COLOR: Color = Color::rgb(0.0, 0.7, 0.0); // green

pub const L: [(i16, i16); 4]  = [(0, 0), (0, 1), (1, 0), (2, 0)];
pub const L_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const L_COLOR: Color = Color::rgb(0.0, 0.0, 0.9); // blue

pub const J: [(i16, i16); 4]  = [(0, 0), (1, 0), (2, 0), (2, 1)];
pub const J_ORIGIN: MatrixPosition = MatrixPosition { x: 1, y: 0};
pub const J_COLOR: Color = Color::rgb(0.9, 0.2, 0.0); // orange


// denotes a block that is part of the current tetromino
#[derive(Debug, Component)]
pub struct TetrominoBlock;


pub fn spawn_tetromino(
    commands: &mut Commands,
    matrix: &Matrix,
    origin: &mut MatrixPosition,
    max_y: i16,
) {
    assert!(matrix.height - 2 > max_y, "Player has lost");

    let tetromino_variant_idx: u16 = rand::thread_rng().gen_range(0..7);
    let (positions, rotation_origin, color) = match tetromino_variant_idx {
        0 => (I, I_ORIGIN, I_COLOR),
        1 => (O, O_ORIGIN, O_COLOR),
        2 => (T, T_ORIGIN, T_COLOR),
        3 => (S, S_ORIGIN, S_COLOR),
        4 => (Z, Z_ORIGIN, Z_COLOR),
        5 => (L, L_ORIGIN, L_COLOR),
        6 => (J, J_ORIGIN, J_COLOR),
        _ => unreachable!(),
//         unimplemented: other variants
    };

    let shift_x = matrix.width / 2 - 1;
    let shift_y = matrix.height - 2;

    *origin = rotation_origin;
    origin.x += shift_x;
    origin.y += shift_y;

    for (x, y) in positions {
        let x = x + shift_x;
        let y = y + shift_y;

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
