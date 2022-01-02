mod movement_types;

use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::rotation::rotate_tetromino;
use crate::heap::HeapEntry;
use crate::tetromino::{Tetromino, TetrominoType};
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
    tetromino_type: Res<TetrominoType>,
    mut reset_lock_delay: ResMut<ResetLockDelay>,
    mut hard_drop: ResMut<HardDrop>,
) {
    // Each block of the tetromino has, appropriately, the `Tetromino` component
    let mut tetromino_pos = tetromino_pos.iter_mut().collect::<Vec<_>>();
    let matrix = matrix.single().unwrap();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        while can_move(
            tetromino_pos.iter(),
            &matrix,
            Move::Y(Y::DownBy1),
            &heap,
        ) {
            tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
        }
        hard_drop.0 = true;
        return;
    }

    // Get movement input
    let mut move_x = if keyboard_input.pressed(KeyCode::J)
        || keyboard_input.pressed(KeyCode::Left)
    {
        Move::X(X::Left)
    } else if keyboard_input.pressed(KeyCode::L)
        || keyboard_input.pressed(KeyCode::Right)
    {
        Move::X(X::Right)
    } else {
        Move::Neutral
    };
    let mut move_y = if keyboard_input.pressed(KeyCode::K)
        || keyboard_input.pressed(KeyCode::Down)
    {
        Move::Y(Y::DownBy1)
    } else {
        Move::Neutral
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
            if !can_move(
                tetromino_pos.iter(),
                &matrix,
                Move::Y(Y::DownBy1),
                &heap,
            ) {
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

    // Get rotation input
    let rotate = if keyboard_input.just_pressed(KeyCode::X) {
        Rotate::Clockwise
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        Rotate::Counterclockwise
    } else {
        Rotate::Neutral
    };
    // Rotation
    rotate_tetromino(
        &mut tetromino_pos,
        *tetromino_type,
        &matrix,
        &heap,
        rotate,
    );

    // Reset lock delay if any input
    reset_lock_delay.0 = !move_x.is_neutral()
        || !move_y.is_neutral()
        || !rotate.is_neutral()
    ;
    hard_drop.0 = false;
}

pub fn can_move<T>(
    mut tetromino_pos: impl Iterator<Item = T>,
    matrix: &Matrix,
    movement: Move,
    heap: &Vec<HeapEntry>,
) -> bool
where
    T: ::core::borrow::Borrow<MatrixPosition>,
{
    tetromino_pos
        .all(|pos| {
            let pos = pos.borrow();
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

            // Check if the neighboring position is occupied in the heap
            let maybe_in_heap = || match heap.get(
                (x + y * matrix.width) as usize
            ) {
                Some(HeapEntry::Vacant) => true,
                _ => false,
            };

            // Invalid 'x' or 'y' will still likely produce a valid index into
            // 'heap'; the index is only accurate if 'x' and 'y' are in bounds
            x >= 0 && x < matrix.width && y >= 0 && maybe_in_heap()
        })
}
