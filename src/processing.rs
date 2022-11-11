use bevy::prelude::*;
use crate::movement::{MoveY, can_move};
use crate::heap::{HeapEntry, add_tetromino_to_heap};
use crate::tetromino::{TetrominoBlock, spawn_tetromino};
use crate::matrix::{Matrix, MatrixPosition};


pub fn processing(
    mut commands: Commands,
    mut heap: ResMut<Vec<HeapEntry>>,
    mut origin: ResMut<MatrixPosition>,
    matrix: Query<&Matrix>,
    tetromino: Query<(Entity, &MatrixPosition), With<TetrominoBlock>>,
) {
    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter()
        .unzip()
    ;
    let matrix = matrix.single();

    if !can_move(&tetromino_pos, &matrix, MoveY::Down1, &heap) {
        // if this is the new highest y value on the heap, then the player
        // may lose (in the case that a new piece can't be spawned)
        let max_y = tetromino_pos
            .iter()
            .map(|pos: &MatrixPosition| pos.y)
            .max()
            .unwrap()
        ;
        add_tetromino_to_heap(
            &mut commands,
            &matrix,
            &mut heap,
            &tetromino_ents,
            &tetromino_pos,
        );
        spawn_tetromino(&mut commands, &matrix, &mut origin, max_y);
    }
}
