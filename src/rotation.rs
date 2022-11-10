use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::TetrominoBlock;
use crate::movement::{MoveNeutral, ResetLockDelay, can_move};
use crate::heap::HeapEntry;
use crate::kb_input::{KeyAction, KeyActions};
use ::core::iter;


#[derive(Copy, Clone, PartialEq)]
pub enum Rotate {
    Clockwise,
    Counterclockwise,
}


pub fn rotation(
    heap: Res<Vec<HeapEntry>>,
    origin: Res<MatrixPosition>,
    keyboard_input: Res<KeyActions>,
    mut reset_lock_delay: ResMut<ResetLockDelay>,
    matrix: Query<&Matrix>,
    mut tetromino_pos: Query<&mut MatrixPosition, With<TetrominoBlock>>,
) {
    // get rotation input
    let clkw = keyboard_input.get_action_state(KeyAction::ClkwJustPressed);
    let cclw = keyboard_input.get_action_state(KeyAction::CclwJustPressed);
    let rotate = match (clkw, cclw) {
        (true, true) | (false, false) => return,
        (true, false) => Rotate::Clockwise,
        (false, true) => Rotate::Counterclockwise,
    };

    let mut tetromino_pos = tetromino_pos.iter_mut().collect::<Vec<_>>();
    // store original positions just in case rotation needs to be reverted
    let prev_pos = tetromino_pos.iter().map(|pos| **pos).collect::<Vec<_>>();
    let matrix = matrix.single();

    basic_rotation(&mut tetromino_pos, rotate, *origin);

    let mut reverted_rotation = false;

    // wall kicks
    if !can_move(&tetromino_pos, &matrix, MoveNeutral, &heap) {
        // relative translations from one kick to the next
        // (according to the wiki ¯\_(ツ)_/¯) T-spins ──────┬───┬
        for try_move in [(1, 0), (1, 0), (-3, 0), (-1, 0), (1, -2)] {
            tetromino_pos.iter_mut().for_each(|pos| **pos += try_move);
            if can_move(&tetromino_pos, &matrix, MoveNeutral, &heap) {
                // successful rotation, so reset lock delay
                reset_lock_delay.set_to(true);
                return;
            }
        }

        // revert rotation
        iter::zip(&mut tetromino_pos, &prev_pos)
            .for_each(|(pos, prev_pos)| **pos = *prev_pos)
        ;
        reverted_rotation = true;
    }

    // if rotation was reverted, fall back to current state
    let current_state = reset_lock_delay.get();
    reset_lock_delay.set_to(current_state | !reverted_rotation);
}

fn basic_rotation(
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    rotate: Rotate,
    origin: MatrixPosition,
) {
    for pos in tetromino_pos {
        let norm_x = pos.x - origin.x;
        let norm_y = pos.y - origin.y;
        match rotate {
            Rotate::Clockwise => {
                pos.x = norm_y;
                pos.y = -norm_x;
            },
            Rotate::Counterclockwise => {
                pos.x = -norm_y;
                pos.y = norm_x;
            },
        }
        **pos += (origin.x, origin.y);
    }
}
