use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};

pub fn rotate_tetromino(
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    rotation_grid_size: i32,
    matrix: &Matrix,
    clockwise: bool,
) {
    let mut offset = 0;

    for pos in &mut *tetromino_pos {
        let x = pos.x;
        let y = pos.y;
        let rotation_grid_size = rotation_grid_size - 1;
    
        if clockwise {
            pos.x = y;
            pos.y = rotation_grid_size - x;
        } else {
            pos.x = rotation_grid_size - y;
            pos.y = x;
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
