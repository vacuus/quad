mod movement_types;

use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::rotation::rotate_tetromino;
use crate::heap::{HeapEntry, add_tetromino_to_heap};
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};
pub use movement_types::*;


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSystemLabel;


pub fn movement(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut movement_timer: ResMut<MovementTimer>,
    mut lock_delay_timer: ResMut<LockDelayTimer>,
    mut heap: ResMut<Vec<HeapEntry>>,
    matrix: Query<&Matrix>,
    mut tetromino: Query<(Entity, &mut MatrixPosition), With<Tetromino>>,
    mut tetromino_type: ResMut<TetrominoType>,
) {
    // Each block of the tetromino has, appropriately, the `Tetromino` component
    let (tetromino_ents, mut tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter_mut()
        .unzip()
    ;
    let matrix = matrix.single().unwrap();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        hard_drop(
            &mut commands,
            &matrix,
            &tetromino_ents,
            &mut tetromino_pos,
            &mut heap,
            &mut materials,
            &mut tetromino_type,
        );
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
    if !can_move(&tetromino_pos, &matrix, move_x, &heap) {
        move_x.set_neutral();
    }
    if !can_move(&tetromino_pos, &matrix, move_y, &heap) {
        move_y.move_up();
        if let Move::Y(Y::DownBy1) = move_y {
            if !can_move(&tetromino_pos, &matrix, move_y, &heap) {
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
    let rotate_clockwise = if keyboard_input.just_pressed(KeyCode::X) {
        Some(true)
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        Some(false)
    } else {
        None
    };
    // Rotation
    if let Some(clockwise) = rotate_clockwise {
        rotate_tetromino(
            &mut tetromino_pos,
            *tetromino_type,
            &matrix,
            &heap,
            clockwise,
        );
    }

    // Reset lock delay if any input
    if move_x != Move::Neutral
        || move_y != Move::Neutral
        || rotate_clockwise.is_some()
    {
        lock_delay_timer.reset();
    }
    if !can_move(&tetromino_pos, &matrix, Move::Y(Y::DownBy1), &heap) {
        // If the tetromino can't move down, commence/continue the lock delay
        lock_delay_timer.tick(time.delta());
        if !lock_delay_timer.just_finished() {
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

fn hard_drop(
    mut commands: &mut Commands,
    matrix: &Matrix,
    tetromino_ents: &Vec<Entity>,
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    mut heap: &mut Vec<HeapEntry>,
    mut materials: &mut Assets<ColorMaterial>,
    mut tetromino_type: &mut TetrominoType,
) {
    while can_move(&tetromino_pos, &matrix, Move::Y(Y::DownBy1), &heap) {
        tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
    }

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

pub fn can_move(
    tetromino_pos: &Vec<Mut<MatrixPosition>>,
    matrix: &Matrix,
    movement: Move,
    heap: &Vec<HeapEntry>,
) -> bool {
    tetromino_pos
        .iter()
        .all(|pos| {
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
