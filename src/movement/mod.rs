mod types;

use bevy::prelude::*;
use ::core::borrow::Borrow;
use crate::grid::{GridSize, GridPos};
use crate::heap::{HeapEntry, Heap};
use crate::piece::{Block, Origin};
use crate::input::{Input, Inputs};
pub use self::types::*;


pub fn movement(
    time: Res<Time>,
    heap: Res<Heap>,
    grid_size: Res<GridSize>,
    inputs: Res<Inputs>,
    mut origin: ResMut<Origin>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut move_x_timer: ResMut<MovementXTimer>,
    mut move_y_timer: ResMut<MovementYTimer>,
    mut block_pos: Query<&mut GridPos, With<Block>>,
) {
    // each block of the piece has, appropriately, the `Block` component
    let mut block_pos = block_pos.iter_mut().collect::<Vec<_>>();
    let grid_width = grid_size.width;

    // hard drop
    if inputs.get_action_state(Input::HardDropJustPressed) {
        while can_move(&block_pos, grid_width, MoveY::Down1, &heap) {
            block_pos.iter_mut().for_each(|pos| pos.y -= 1);
        }
        return;
    }

    // get movement input
    let (mut move_x, mut move_y) = {
        use self::Input::*;


        let left_press = inputs.get_action_state(LeftPressed);
        let right_press = inputs.get_action_state(RightPressed);
        let move_x = match (left_press, right_press) {
            (true, true) | (false, false) => MoveX::Neutral,
            (true, false) => MoveX::Left,
            (false, true) => MoveX::Right,
        };

        if inputs.get_action_state(SoftDropPressed) {
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
    if !can_move(&block_pos, grid_width, move_x, &heap) {
        move_x.set_neutral();
    }
    if !can_move(&block_pos, grid_width, move_y, &heap) {
        move_y.move_up();
        if move_y == MoveY::Down1
            && !can_move(&block_pos, grid_width, MoveY::Down1, &heap)
        {
            move_y.set_neutral();
        }
    }

    let offset = (move_x, move_y).to_offset();
    // apply movement
    block_pos.iter_mut().for_each(|pos| { **pos += offset; });
    origin.pos += offset;
}

pub fn can_move<Pos, Mov>(
    block_pos: impl IntoIterator<Item = Pos>,
    grid_width: i16,
    movement: Mov,
    heap: &Heap,
) -> bool
where
    Pos: Borrow<GridPos>,
    Mov: MoveOffset,
{
    let heap = &heap.blocks;
    let offset = <Mov as MoveOffset>::to_offset(&movement);

    block_pos
        .into_iter()
        .map(|pos| *<Pos as Borrow<GridPos>>::borrow(&pos))
        .all(|mut pos| {
            // get neighboring position in relevant direction
            pos += offset;

            // invalid `x` or `y` will still likely produce a valid index into
            // `heap`; the index is only accurate if `x` and `y` are in bounds
            pos.x >= 0 && pos.x < grid_width && pos.y >= 0
                && match heap.get((pos.x + pos.y * grid_width) as usize)
            {
                Some(HeapEntry::Vacant) | None => true,
                Some(HeapEntry::Occupied) => false,
            }
        })
}
