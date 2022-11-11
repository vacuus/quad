use bevy::prelude::*;
use crate::grid::{GridSize, GridPos};
use crate::spawn::{Block, SpawnEvent};
use crate::movement::{MoveY, can_move};


#[derive(Clone)]
pub enum HeapEntry {
    Vacant,
    Occupied,
}


pub fn lock(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    mut max_y: ResMut<i16>,
    mut heap: ResMut<Vec<HeapEntry>>,
    mut spawn_notify: EventWriter<SpawnEvent>,
    tetromino: Query<(Entity, &GridPos), With<Block>>,
) {
    let grid_width = grid_size.width;

    let (block_entities, block_pos): (Vec<_>, Vec<&GridPos>) =
        tetromino.iter().unzip()
    ;

    if can_move(&block_pos, grid_width, MoveY::Down1, &heap) {
        return;
    }

    // if this is the new highest y value on the heap, then the player
    // may lose (in the case that a new piece can't be spawned)
    *max_y = block_pos.iter().map(|pos| pos.y).max().unwrap();
    spawn_notify.send(SpawnEvent);

    block_entities
        .into_iter()
        .for_each(|entity| {
            commands.entity(entity).remove::<Block>();
        })
    ;
    block_pos
        .into_iter()
        .for_each(|pos: &GridPos| {
            // mark position in heap as occupied
            heap[(pos.x + pos.y * grid_width) as usize] = HeapEntry::Occupied;
        })
    ;
}
