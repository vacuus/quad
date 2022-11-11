use bevy::prelude::*;
use crate::grid::{GridSize, GridPos};
use crate::tetromino::TetrominoBlock;
use crate::movement::{MoveNeutral, can_move};
use crate::heap::HeapEntry;
use crate::input::{KeyAction, KeyActions};
use ::core::iter;


#[derive(Copy, Clone, PartialEq)]
pub enum Rotate {
    Clockwise,
    Counterclockwise,
}


pub fn rotation(
    heap: Res<Vec<HeapEntry>>,
    grid_size: Res<GridSize>,
    origin: Res<GridPos>,
    keyboard_input: Res<KeyActions>,
    mut tetromino_pos: Query<&mut GridPos, With<TetrominoBlock>>,
) {
    let grid_width = grid_size.width;

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

    basic_rotation(&mut tetromino_pos, rotate, *origin);

    // wall kicks
    if !can_move(&tetromino_pos, grid_width, MoveNeutral, &heap) {
        // relative translations from one kick to the next
        // (according to the wiki ¯\_(ツ)_/¯) T-spins ──────┬───┬
        for try_move in [(1, 0), (1, 0), (-3, 0), (-1, 0), (1, -2)] {
            tetromino_pos.iter_mut().for_each(|pos| **pos += try_move);
            if can_move(&tetromino_pos, grid_width, MoveNeutral, &heap) {
                // kick was successful
                return;
            }
        }

        // revert rotation
        iter::zip(&mut tetromino_pos, &prev_pos)
            .for_each(|(pos, prev_pos)| **pos = *prev_pos)
        ;
    }
}

fn basic_rotation(
    tetromino_pos: &mut Vec<Mut<GridPos>>,
    rotate: Rotate,
    origin: GridPos,
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
