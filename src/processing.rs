use bevy::prelude::*;
use crate::movement::{MoveY, can_move};
use crate::heap::HeapEntry;
use crate::tetromino::{TetrominoBlock, LockEvent};
use crate::matrix::{Matrix, MatrixPosition};


pub fn processing(
    mut max_y: ResMut<i16>,
    heap: Res<Vec<HeapEntry>>,
    mut lock_notify: EventWriter<LockEvent>,
    matrix: Query<&Matrix>,
    tetromino: Query<&MatrixPosition, With<TetrominoBlock>>,
) {
    let tetromino_pos = tetromino.iter().collect::<Vec<_>>();
    let matrix = matrix.single();

    if !can_move(&tetromino_pos, &matrix, MoveY::Down1, &heap) {
        // if this is the new highest y value on the heap, then the player
        // may lose (in the case that a new piece can't be spawned)
        *max_y = tetromino_pos.iter().map(|pos| pos.y).max().unwrap();
        lock_notify.send(LockEvent);
    }
}
