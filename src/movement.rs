mod types;

use bevy::prelude::*;
use crate::grid::{GridSize, GridPos};
use crate::heap::HeapEntry;
use crate::tetromino::TetrominoBlock;
use crate::input::{KeyAction, KeyActions};
pub use self::types::*;


pub fn movement(
    time: Res<Time>,
    heap: Res<Vec<HeapEntry>>,
    grid_size: Res<GridSize>,
    keyboard_input: Res<KeyActions>,
    mut origin: ResMut<GridPos>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut move_x_timer: ResMut<MovementXTimer>,
    mut move_y_timer: ResMut<MovementYTimer>,
    mut tetromino_pos: Query<&mut GridPos, With<TetrominoBlock>>,
) {
    // each block of the tetromino has, appropriately, the `Tetromino` component
    let mut tetromino_pos = tetromino_pos.iter_mut().collect::<Vec<_>>();

    // hard drop
    if keyboard_input.get_action_state(KeyAction::HardDropJustPressed) {
        while can_move(&tetromino_pos, &grid_size, MoveY::Down1, &heap) {
            tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
        }
        return;
    }

    // get movement input
    let (mut move_x, mut move_y) = {
        use self::KeyAction::*;


        let left_press = keyboard_input.get_action_state(LeftPressed);
        let right_press = keyboard_input.get_action_state(RightPressed);
        let move_x = match (left_press, right_press) {
            (true, true) | (false, false) => MoveX::Neutral,
            (true, false) => MoveX::Left,
            (false, true) => MoveX::Right,
        };

        if keyboard_input.get_action_state(SoftDropPressed) {
            (move_x, MoveY::Down1)
        } else {
            (move_x, MoveY::Neutral)
        }
    };

    // only allow movement every so often
    move_x_timer.tick(time.delta());
    if move_x_timer.just_finished() {
        move_x_timer.reset();
    } else {
        // ignore movement input
        move_x.set_neutral();
    }
    move_y_timer.tick(time.delta());
    if move_y_timer.just_finished() {
        move_y_timer.reset();
    } else {
        move_y.set_neutral();
    }

    // gravity
    gravity_timer.tick(time.delta());
    if gravity_timer.just_finished() {
        move_y.move_down();
        gravity_timer.reset();
    }

    // check if movement is legal
    if !can_move(&tetromino_pos, &grid_size, move_x, &heap) {
        move_x.set_neutral();
    }
    if !can_move(&tetromino_pos, &grid_size, move_y, &heap) {
        move_y.move_up();
        if move_y == MoveY::Down1
            && !can_move(&tetromino_pos, &grid_size, MoveY::Down1, &heap)
        {
            move_y.set_neutral();
        }
    }

    let offset = (move_x, move_y).to_offset();
    // apply movement
    tetromino_pos.iter_mut().for_each(|pos| { **pos += offset; });
    *origin += offset;
}


use ::core::borrow::Borrow;


pub fn can_move<Pos, Mov>(
    tetromino_pos: impl IntoIterator<Item = Pos>,
    grid_size: &GridSize,
    movement: Mov,
    heap: &Vec<HeapEntry>,
) -> bool
where
    Pos: Borrow<GridPos>,
    Mov: MoveOffset,
{
    let offset = <Mov as MoveOffset>::to_offset(&movement);

    tetromino_pos
        .into_iter()
        .map(|pos| *<Pos as Borrow<GridPos>>::borrow(&pos))
        .all(|mut pos| {
            // get neighboring position in relevant direction
            pos += offset;

            // invalid `x` or `y` will still likely produce a valid index into
            // `heap`; the index is only accurate if `x` and `y` are in bounds
            (pos.x >= 0) & (pos.x < grid_size.width) & (pos.y >= 0)
                && match heap.get((pos.x + pos.y * grid_size.width) as usize)
            {
                Some(HeapEntry::Vacant) => true,
                _ => false,
            }
        })
}
