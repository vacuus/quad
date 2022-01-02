use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::Tetromino;

#[derive(Clone)]
pub enum HeapEntry {
    Vacant,
    Occupied,
}

pub fn add_tetromino_to_heap(
    commands: &mut Commands,
    tetromino_ents: &Vec<Entity>,
    heap: &mut Vec<HeapEntry>,
    tetromino_pos: &Vec<MatrixPosition>,
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
            heap[(pos.x + pos.y * matrix.width) as usize] = HeapEntry::Occupied;
        })
    ;
}
