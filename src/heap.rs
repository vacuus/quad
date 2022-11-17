use bevy::prelude::*;
use bevy::app::AppExit;
use crate::grid::{GridSize, GridPos};
use crate::piece::{Block, SpawnEvent};
use crate::movement::{MoveY, can_move};


#[derive(Resource)]
pub struct Heap {
    pub blocks: Vec<HeapEntry>,
}

#[derive(Clone)]
pub enum HeapEntry {
    Vacant,
    Occupied,
}


pub fn lock(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    mut heap: ResMut<Heap>,
    mut lose_notify: EventWriter<AppExit>,
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

    if block_pos.iter().map(|pos| pos.y).any(|y| y >= grid_size.height) {
        eprintln!("You lost");
        lose_notify.send(AppExit);
        return;
    }

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
            let idx = pos.x + pos.y * grid_width;
            // mark position in heap as occupied
            heap.blocks[idx as usize] = HeapEntry::Occupied;
        })
    ;
}
