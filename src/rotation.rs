use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::tetromino::{Tetromino, TetrominoType};
use crate::movement::{MoveNeutral, ResetLockDelay, can_move};
use crate::heap::HeapEntry;
use crate::kb_input::{KeyAction, KeyActions};


#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct RotationSystem;

#[derive(Copy, Clone, PartialEq)]
pub enum Rotate {
    Clockwise,
    Counterclockwise,
}


pub fn rotation(
    heap: Res<Vec<HeapEntry>>,
    keyboard_input: Res<KeyActions>,
    tetromino_type: Res<TetrominoType>,
    mut reset_lock_delay: ResMut<ResetLockDelay>,
    matrix: Query<&Matrix>,
    mut tetromino_pos: Query<&mut MatrixPosition, With<Tetromino>>,
) {
    // get rotation input
    let rotate = if keyboard_input.get_action(KeyAction::ClockwiseJustPressed) {
        Rotate::Clockwise
    } else if keyboard_input.get_action(KeyAction::CounterclockwiseJustPressed)
    {
        Rotate::Counterclockwise
    } else {
        return;
    };
    // reset lock delay if any input
    reset_lock_delay.set_to(true);

    let mut tetromino_pos = tetromino_pos.iter_mut().collect::<Vec<_>>();
    let matrix = matrix.single();
    // store original positions just in case rotation needs to be reverted
    let prev_positions = tetromino_pos
        .iter()
        .map(|pos| **pos)
        .collect::<Vec<_>>()
    ;

    basic_rotation(&mut tetromino_pos, *tetromino_type, rotate);

    // wall kicks
    if !can_move(tetromino_pos.iter(), &matrix, MoveNeutral, &heap) {
        // relative translations from one kick to the next
        // (according to the wiki ¯\_(ツ)_/¯) T-spins ──────┬───┬
        let try_moves = [(1, 0), (1, 0), (-3, 0), (-1, 0), (1, -2)];
        for try_move in try_moves {
            tetromino_pos.iter_mut().for_each(|pos| **pos += try_move);
            if can_move(tetromino_pos.iter(), &matrix, MoveNeutral, &heap) {
                return;
            }
        }

        // revert rotation
        tetromino_pos
            .iter_mut()
            .zip(&prev_positions)
            .for_each(|(pos, prev_pos)| **pos = *prev_pos)
        ;
        reset_lock_delay.set_to(false);
    }
}

fn basic_rotation(
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    tetromino_type: TetrominoType,
    rotate: Rotate,
) {
    use TetrominoType::*;

    let rotation_grid_size = match tetromino_type {
        I => 4,
        O => 2,
        T | Z | S | L | J => 3,
    };

    let min_x = tetromino_pos.iter().map(|pos| pos.x).min().unwrap();
    let min_y = tetromino_pos.iter().map(|pos| pos.y).min().unwrap();

    let center_x = min_x + rotation_grid_size / 2;
    let center_y = min_y + rotation_grid_size / 2;

    for pos in &mut *tetromino_pos {
        let x = pos.x - center_x;
        let y = pos.y - center_y;

        if rotate == Rotate::Clockwise {
            pos.x = y + center_x;
            pos.y = -x + center_y;
        } else {
            pos.x = -y + center_x;
            pos.y = x + center_y;
        }
    }
}
