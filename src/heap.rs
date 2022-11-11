use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::{TetrominoBlock, LockEvent};


#[derive(Clone)]
pub enum HeapEntry {
    Vacant,
    Occupied,
}


pub fn lock(
    mut commands: Commands,
    mut heap: ResMut<Vec<HeapEntry>>,
    lock_update: EventReader<LockEvent>,
    matrix: Query<&Matrix>,
    tetromino: Query<(Entity, &MatrixPosition), With<TetrominoBlock>>,
) {
    if lock_update.is_empty() {
        return;
    }
    lock_update.clear();

    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter()
        .unzip()
    ;
    let matrix_width = matrix.single().width;

    tetromino_ents
        .iter()
        .for_each(|&entity| {
            commands.entity(entity).remove::<TetrominoBlock>();
        })
    ;
    tetromino_pos
        .iter()
        .for_each(|pos: &MatrixPosition| {
            // mark position in heap as occupied
            heap[(pos.x + pos.y * matrix_width) as usize] = HeapEntry::Occupied;
        })
    ;
}
