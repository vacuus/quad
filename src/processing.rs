use bevy::prelude::*;
use crate::movement::{
    Move,
    Y,
    LockDelayTimer,
    HardDropOccurred,
    ResetLockDelay,
    can_move,
};
use crate::heap::{HeapEntry, add_tetromino_to_heap};
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};
use crate::matrix::{Matrix, MatrixPosition};


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ProcessingSystem;


pub fn processing(
    mut commands: Commands,
    time: Res<Time>,
    reset_lock_delay: Res<ResetLockDelay>,
    hard_drop_occurred: Res<HardDropOccurred>,
    mut heap: ResMut<Vec<HeapEntry>>,
    mut tetromino_type: ResMut<TetrominoType>,
    mut lock_delay_timer: ResMut<LockDelayTimer>,
    matrix: Query<&Matrix>,
    tetromino: Query<(Entity, &MatrixPosition), With<Tetromino>>,
) {
    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter()
        .unzip()
    ;
    let matrix = matrix.single();

    if reset_lock_delay.get() {
        lock_delay_timer.reset();
    }
    if !can_move(tetromino_pos.iter(), &matrix, Move::Y(Y::Down1), &heap) {
        // If the tetromino can't move down, commence/continue the lock delay
        lock_delay_timer.tick(time.delta());
        if !hard_drop_occurred.get() && !lock_delay_timer.just_finished() {
            return;
        }
        lock_delay_timer.reset();

        // Revert movement and add tetromino to heap
        add_tetromino_to_heap(
            &mut commands,
            &matrix,
            &mut heap,
            &tetromino_ents,
            &tetromino_pos,
        );
        spawn_tetromino(&mut commands, &matrix, &mut tetromino_type);
    }
}
