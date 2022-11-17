mod defaults;

use bevy::prelude::*;
use rand::Rng;
use crate::grid::{GridSize, GridPos};
use crate::BLOCK_SIZE;
use self::defaults::*;


// denotes a block that is part of the current piece
#[derive(Debug, Component)]
pub struct Block;

// the current piece has been locked, and a new piece will be spawned
pub struct SpawnEvent;

#[derive(Clone, Copy, Resource)]
pub struct Origin {
    pub pos: GridPos,
    pub mode: OriginMode,
}

#[derive(Clone, Copy)]
pub enum OriginMode {
    // - the origin lies at an intersection of grid lines; the given position
    // is of the grid cell that has the origin at its bottom left corner 
    // - of the builtin pieces, I and O have such "point-centered" origins
    PointCentered,
    // the origin lies squarely on the grid cell given by the position
    BlockCentered,
}


pub fn spawn(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    mut origin: ResMut<Origin>,
    spawn_update: EventReader<SpawnEvent>,
) {
    if spawn_update.is_empty() {
        return;
    }
    spawn_update.clear();

    let piece_variant_idx: u16 = rand::thread_rng().gen_range(0..7);
    let (positions, origin_mode, color) = match piece_variant_idx {
        0 => (I_POS, I_ORIGIN_MODE, I_COLOR),
        1 => (O_POS, O_ORIGIN_MODE, O_COLOR),
        2 => (T_POS, T_ORIGIN_MODE, T_COLOR),
        3 => (S_POS, S_ORIGIN_MODE, S_COLOR),
        4 => (Z_POS, Z_ORIGIN_MODE, Z_COLOR),
        5 => (L_POS, L_ORIGIN_MODE, L_COLOR),
        6 => (J_POS, J_ORIGIN_MODE, J_COLOR),
        _ => unreachable!(),
//         unimplemented: other variants
    };

    let shift_x = grid_size.width / 2 - 1;
    let shift_y = grid_size.height;

    origin.pos = GridPos { x: shift_x, y: shift_y };
    origin.mode = origin_mode;

    for (x, y) in positions {
        let pos = GridPos {
            x: x + shift_x,
            y: y + shift_y,
        };

        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(BLOCK_SIZE)),
                        color,
                        ..Sprite::default()
                    },
                    transform: Transform::from_translation(
                        Vec3::new(
                            x as f32 * BLOCK_SIZE,
                            y as f32 * BLOCK_SIZE,
                            1.0,
                        ),
                    ),
                    ..SpriteBundle::default()
                },
                Block,
                pos,
            ))
        ;
    }
}
