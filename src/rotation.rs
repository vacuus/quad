use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::TetrominoType;
use crate::movement::{Move, Rotate, can_move};
use crate::heap::HeapEntry;

pub fn rotate_tetromino(
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    tetromino_type: TetrominoType,
    matrix: &Matrix,
    heap: &Vec<HeapEntry>,
    rotate: Rotate,
) {
    if rotate == Rotate::Neutral {
        return;
    }

    // Store original positions just in case revert is needed
    let prev_positions = tetromino_pos
        .iter()
        .map(|pos| **pos)
        .collect::<Vec<_>>()
    ;

    basic_rotation(tetromino_pos, tetromino_type, &matrix, rotate);

    // Wall kicks
    if !can_move(&tetromino_pos, &matrix, Move::Neutral, &heap) {
        // T spins: (1, -2)
        let try_moves = [(1, 0), (2, 0), (-1, 0), (-2, 0), (-1, -2)];
        for try_move in try_moves {
            tetromino_pos.iter_mut().for_each(|pos| **pos += try_move);
            if can_move(&tetromino_pos, &matrix, Move::Neutral, &heap) {
                return;
            }
        }

        tetromino_pos
            .iter_mut()
            .zip(prev_positions)
            .for_each(|(pos, prev_pos)| **pos = prev_pos)
        ;
    }
}

fn basic_rotation(
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    tetromino_type: TetrominoType,
    matrix: &Matrix,
    rotate: Rotate,
) {
    use TetrominoType::*;

    let rotation_grid_size = match tetromino_type {
        I => 4,
        O => 2,
        T | Z | S | L | J => 3,
    };

    let min_x = tetromino_pos.iter().map(|pos| pos.x).min().unwrap();
    let min_y = tetromino_pos.iter().map(|pos| pos.y).min().unwrap();

    let center_x = min_x + rotation_grid_size / 2;
    let center_y = min_y + rotation_grid_size / 2;

    let mut offset = 0;

    for pos in &mut *tetromino_pos {
        let x = pos.x - center_x;
        let y = pos.y - center_y;

        if rotate == Rotate::Clockwise {
            pos.x = y + center_x;
            pos.y = -x + center_y;
        } else {
            pos.x = -y + center_x;
            pos.y = x + center_y;
        }

        if pos.x < 0 {
            offset = offset.max(-pos.x);
        } else if pos.x >= matrix.width {
            offset = offset.min(matrix.width - pos.x - 1);
        }
    }

    for pos in &mut *tetromino_pos {
        pos.x += offset;
    }
}
