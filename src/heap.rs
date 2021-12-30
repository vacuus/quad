use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::Tetromino;

pub fn add_tetromino_to_heap(
    commands: &mut Commands,
    tetromino_ents: &Vec<Entity>,
    heap: &mut Vec<Option<()>>,
    tetromino_pos: &Vec<Mut<MatrixPosition>>,
    matrix: &Matrix,
) {
    tetromino_ents
        .iter()
        .for_each(|&entity| {
            commands
                .entity(entity)
                .remove::<Tetromino>()
            ;
        })
    ;
    tetromino_pos
        .iter()
        .for_each(|pos| {
            // Mark position as occupied in heap
            heap[(pos.x + pos.y * matrix.width) as usize] = Some(());
        })
    ;
}
