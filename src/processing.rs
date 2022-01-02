use bevy::prelude::*;
use crate::movement::{
    Move,
    Y,
    LockDelayTimer,
    HardDrop,
    ResetLockDelay,
    can_move
};
use crate::heap::{HeapEntry, add_tetromino_to_heap};
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};
use crate::matrix::{Matrix, MatrixPosition};


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ProcessingSystem;


pub fn processing(
    mut commands: Commands,
    time: Res<Time>,
    mut lock_delay_timer: ResMut<LockDelayTimer>,
    mut tetromino_type: ResMut<TetrominoType>,
    matrix: Query<&Matrix>,
    mut heap: ResMut<Vec<HeapEntry>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    reset_lock_delay: Res<ResetLockDelay>,
    hard_drop: Res<HardDrop>,
    mut tetromino: Query<(Entity, &mut MatrixPosition), With<Tetromino>>,
) {
    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter_mut()
        .unzip()
    ;
    let matrix = matrix.single().unwrap();

    if reset_lock_delay.0 {
        lock_delay_timer.reset();
    }
    if !can_move(&tetromino_pos, &matrix, Move::Y(Y::DownBy1), &heap) {
        // If the tetromino can't move down, commence/continue the lock delay
        lock_delay_timer.tick(time.delta());
        if !hard_drop.0 && !lock_delay_timer.just_finished() {
            return;
        }
        lock_delay_timer.reset();
        // Revert movement and add tetromino to heap
        add_tetromino_to_heap(
            &mut commands,
            &tetromino_ents,
            &mut heap,
            &tetromino_pos,
            &matrix,
        );
        spawn_tetromino(
            &mut commands,
            &matrix,
            &mut materials,
            &mut tetromino_type,
        );
    }
}
