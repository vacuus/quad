use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::{TetrominoBlock, LockEvent};
use crate::movement::{MoveY, can_move};


#[derive(Clone)]
pub enum HeapEntry {
    Vacant,
    Occupied,
}


pub fn lock(
    mut commands: Commands,
    mut max_y: ResMut<i16>,
    mut heap: ResMut<Vec<HeapEntry>>,
    mut lock_notify: EventWriter<LockEvent>,
    matrix: Query<&Matrix>,
    tetromino: Query<(Entity, &MatrixPosition), With<TetrominoBlock>>,
) {
    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<&MatrixPosition>) =
        tetromino.iter().unzip()
    ;
    let matrix = matrix.single();

    if can_move(&tetromino_pos, &matrix, MoveY::Down1, &heap) {
        return;
    }

    // if this is the new highest y value on the heap, then the player
    // may lose (in the case that a new piece can't be spawned)
    *max_y = tetromino_pos.iter().map(|pos| pos.y).max().unwrap();
    lock_notify.send(LockEvent);

    let matrix_width = matrix.width;

    tetromino_ents
        .into_iter()
        .for_each(|entity| {
            commands.entity(entity).remove::<TetrominoBlock>();
        })
    ;
    tetromino_pos
        .into_iter()
        .for_each(|pos: &MatrixPosition| {
            // mark position in heap as occupied
            heap[(pos.x + pos.y * matrix_width) as usize] = HeapEntry::Occupied;
        })
    ;
}
