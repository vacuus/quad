mod movement_types;

use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::heap::HeapEntry;
use crate::tetromino::Tetromino;
use crate::kb_input::{KeyAction, KeyActions};
pub use movement_types::*;


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSystem;


pub fn movement(
    time: Res<Time>,
    heap: Res<Vec<HeapEntry>>,
    keyboard_input: Res<KeyActions>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut movement_timer: ResMut<MovementTimer>,
    mut reset_lock_delay: ResMut<ResetLockDelay>,
    matrix: Query<&Matrix>,
    mut tetromino_pos: Query<&mut MatrixPosition, With<Tetromino>>,
) {
    // each block of the tetromino has, appropriately, the `Tetromino` component
    let mut tetromino_pos = tetromino_pos.iter_mut().collect::<Vec<_>>();
    let matrix = matrix.single();

    // hard drop
    if keyboard_input.get_action(KeyAction::HardDropJustPressed) {
        while can_move(tetromino_pos.iter(), &matrix, MoveY::Down1, &heap) {
            tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
        }
        return;
    }

    // get movement input
    let (mut move_x, mut move_y) = {
        let move_x = if keyboard_input.get_action(KeyAction::LeftPressed) {
            MoveX::Left
        } else if keyboard_input.get_action(KeyAction::RightPressed) {
            MoveX::Right
        } else {
            MoveX::Neutral
        };
        let move_y = if keyboard_input.get_action(KeyAction::DownPressed) {
            MoveY::Down1
        } else {
            MoveY::Neutral
        };

        (move_x, move_y)
    };

    // only allow movement every so often
    movement_timer.tick(time.delta());
    if movement_timer.just_finished() {
        movement_timer.reset();
    } else {
        // ignore movement input, but gravity can still take effect
        move_x.set_neutral();
        move_y.set_neutral();
    }

    // gravity
    gravity_timer.tick(time.delta());
    if gravity_timer.just_finished() {
        move_y.move_down();
        gravity_timer.reset();
    }

    // check if movement is legal
    if !can_move(tetromino_pos.iter(), &matrix, move_x, &heap) {
        move_x.set_neutral();
    }
    if !can_move(tetromino_pos.iter(), &matrix, move_y, &heap) {
        move_y.move_up();
        if move_y == MoveY::Down1 {
            if !can_move(tetromino_pos.iter(), &matrix, MoveY::Down1, &heap) {
                move_y.set_neutral();
            }
        }
    }

    // apply movement
    tetromino_pos.iter_mut().for_each(|pos| {
        **pos += (move_x, move_y).to_offset();
    });

    // reset lock delay if any input
    reset_lock_delay.set_to(!move_x.is_neutral() | !move_y.is_neutral());
}


use ::core::borrow::Borrow;


pub fn can_move<T, M>(
    mut tetromino_pos: impl Iterator<Item = T>,
    matrix: &Matrix,
    movement: M,
    heap: &Vec<HeapEntry>,
) -> bool
where
    T: Borrow<MatrixPosition>,
    M: MoveOffset,
{
    tetromino_pos
        .all(|pos| {
            let mut pos = *<T as Borrow<MatrixPosition>>::borrow(&pos);
            // get neighboring position in relevant direction
            pos += <M as MoveOffset>::to_offset(&movement);
            let (x, y) = (pos.x, pos.y);

            // invalid 'x' or 'y' will still likely produce a valid index into
            // 'heap'; the index is only accurate if 'x' and 'y' are in bounds
            (x >= 0) & (x < matrix.width) & (y >= 0) && match heap.get(
                (x + y * matrix.width) as usize
            ) {
                Some(HeapEntry::Vacant) => true,
                _ => false,
            }
        })
}
