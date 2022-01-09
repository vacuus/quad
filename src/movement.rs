mod movement_types;

use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::heap::HeapEntry;
use crate::tetromino::Tetromino;
pub use movement_types::*;


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSystem;


pub fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut movement_timer: ResMut<MovementTimer>,
    heap: Res<Vec<HeapEntry>>,
    matrix: Query<&Matrix>,
    mut tetromino_pos: Query<&mut MatrixPosition, With<Tetromino>>,
    mut reset_lock_delay: ResMut<ResetLockDelay>,
    mut hard_drop: ResMut<HardDropOccurred>,
) {
    const MOVE_DOWN_BY_1: Move = Move::Y(Y::DownBy1);

    // Each block of the tetromino has, appropriately, the `Tetromino` component
    let mut tetromino_pos = tetromino_pos.iter_mut().collect::<Vec<_>>();
    let matrix = matrix.single();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        while can_move(tetromino_pos.iter(), &matrix, MOVE_DOWN_BY_1, &heap) {
            tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
        }
        hard_drop.set_to(true);
        return;
    }

    // Get movement input
    let (mut move_x, mut move_y) = {
        use KeyCode::{J, K, L, Left, Right, Down};

        let move_x = if keyboard_input.any_pressed([J, Left]) {
            Move::X(X::Left)
        } else if keyboard_input.any_pressed([L, Right]) {
            Move::X(X::Right)
        } else {
            Move::Neutral
        };
        let move_y = if keyboard_input.any_pressed([K, Down]) {
            Move::Y(Y::DownBy1)
        } else {
            Move::Neutral
        };

        (move_x, move_y)
    };

    // Only allow movement every so often
    movement_timer.tick(time.delta());
    if movement_timer.just_finished() {
        movement_timer.reset();
    } else {
        // Ignore movement input, but gravity can still take effect
        move_x.set_neutral();
        move_y.set_neutral();
    }

    // Gravity
    gravity_timer.tick(time.delta());
    if gravity_timer.just_finished() {
        move_y.move_down();
        gravity_timer.reset();
    }

    // Check if movement is legal
    if !can_move(tetromino_pos.iter(), &matrix, move_x, &heap) {
        move_x.set_neutral();
    }
    if !can_move(tetromino_pos.iter(), &matrix, move_y, &heap) {
        move_y.move_up();
        if let Move::Y(Y::DownBy1) = move_y {
            if !can_move(tetromino_pos.iter(), &matrix, MOVE_DOWN_BY_1, &heap) {
                move_y.set_neutral();
            }
        }
    }

    // Apply movement
    tetromino_pos.iter_mut().for_each(|pos| {
        pos.x += match move_x {
            Move::Neutral => 0,
            Move::X(X::Left) => -1,
            Move::X(X::Right) => 1,
            _ => unreachable!(),
        };
        pos.y += match move_y {
            Move::Neutral => 0,
            Move::Y(Y::DownBy1) => -1,
            Move::Y(Y::DownBy2) => -2,
            _ => unreachable!(),
        };
    });

    // Reset lock delay if any input
    reset_lock_delay.set_to(!move_x.is_neutral() | !move_y.is_neutral());
    hard_drop.set_to(false);
}


use ::core::borrow::Borrow;


pub fn can_move<T>(
    mut tetromino_pos: impl Iterator<Item = T>,
    matrix: &Matrix,
    movement: Move,
    heap: &Vec<HeapEntry>,
) -> bool
where
    T: Borrow<MatrixPosition>,
{
    tetromino_pos
        .all(|pos| {
            let pos = <T as Borrow<MatrixPosition>>::borrow(&pos);
            // Get neighboring position in relevant direction
            let (x, y) = match movement {
                Move::Y(Y::DownBy1) => (pos.x, pos.y - 1),
                Move::Y(Y::DownBy2) => (pos.x, pos.y - 2),
                Move::X(X::Left) => (pos.x - 1, pos.y),
                Move::X(X::Right) => (pos.x + 1, pos.y),
                Move::Neutral => (pos.x, pos.y),
                // Hard drop isn't a possibility at this point
                Move::Y(Y::HardDrop) => return true,
            };

            // Invalid 'x' or 'y' will still likely produce a valid index into
            // 'heap'; the index is only accurate if 'x' and 'y' are in bounds
            (x >= 0) & (x < matrix.width) & (y >= 0) && match heap.get(
                (x + y * matrix.width) as usize
            ) {
                Some(HeapEntry::Vacant) => true,
                _ => false,
            }
        })
}
