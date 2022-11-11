use bevy::prelude::*;
use crate::grid::{GridSize, GridPos};
use crate::tetromino::{TetrominoBlock, SpawnEvent};
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
    tetromino: Query<(Entity, &GridPos), With<TetrominoBlock>>,
) {
    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<&GridPos>) =
        tetromino.iter().unzip()
    ;

    if can_move(&tetromino_pos, &grid_size, MoveY::Down1, &heap) {
        return;
    }

    // if this is the new highest y value on the heap, then the player
    // may lose (in the case that a new piece can't be spawned)
    *max_y = tetromino_pos.iter().map(|pos| pos.y).max().unwrap();
    spawn_notify.send(SpawnEvent);

    let matrix_width = grid_size.width;

    tetromino_ents
        .into_iter()
        .for_each(|entity| {
            commands.entity(entity).remove::<TetrominoBlock>();
        })
    ;
    tetromino_pos
        .into_iter()
        .for_each(|pos: &GridPos| {
            // mark position in heap as occupied
            heap[(pos.x + pos.y * matrix_width) as usize] = HeapEntry::Occupied;
        })
    ;
}
