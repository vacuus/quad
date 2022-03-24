use bevy::prelude::*;
use crate::movement::{MoveY, LockDelayTimer, ResetLockDelay, can_move};
use crate::heap::{HeapEntry, add_tetromino_to_heap};
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};
use crate::matrix::{Matrix, MatrixPosition};
use crate::kb_input::{KeyAction, KeyActions};


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ProcessingSystem;


pub fn processing(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<KeyActions>,
    reset_lock_delay: Res<ResetLockDelay>,
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
    if !can_move(tetromino_pos.iter(), &matrix, MoveY::Down1, &heap) {
        // If the tetromino can't move down, commence/continue the lock delay
        lock_delay_timer.tick(time.delta());
        if !keyboard_input.get_action(KeyAction::HardDropJustPressed)
            && !lock_delay_timer.just_finished()
        {
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
