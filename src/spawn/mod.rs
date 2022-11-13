mod defaults;

use bevy::prelude::*;
use bevy::app::AppExit;
use rand::Rng;
use crate::grid::{GridSize, GridPos};
use crate::BLOCK_SIZE;
use self::defaults::*;


// denotes a block that is part of the current piece
#[derive(Debug, Component)]
pub struct Block;

// the current piece has been locked, and a new piece will be spawned
pub struct SpawnEvent;

#[derive(Resource)]
pub struct MaxY {
    pub val: i16,
}


pub fn spawn(
    mut commands: Commands,
    max_y: Res<MaxY>,
    grid_size: Res<GridSize>,
    mut origin: ResMut<GridPos>,
    spawn_update: EventReader<SpawnEvent>,
    mut app_exit_notify: EventWriter<AppExit>,
) {
    if spawn_update.is_empty() {
        return;
    }
    spawn_update.clear();

    if max_y.val >= grid_size.height - 2 {
        eprintln!("You lost");
        app_exit_notify.send(AppExit);
    }

    let piece_variant_idx: u16 = rand::thread_rng().gen_range(0..7);
    let (positions, rotation_origin, color) = match piece_variant_idx {
        0 => (I_POS, I_ORIGIN, I_COLOR),
        1 => (O_POS, O_ORIGIN, O_COLOR),
        2 => (T_POS, T_ORIGIN, T_COLOR),
        3 => (S_POS, S_ORIGIN, S_COLOR),
        4 => (Z_POS, Z_ORIGIN, Z_COLOR),
        5 => (L_POS, L_ORIGIN, L_COLOR),
        6 => (J_POS, J_ORIGIN, J_COLOR),
        _ => unreachable!(),
//         unimplemented: other variants
    };

    let shift_x = grid_size.width / 2 - 1;
    let shift_y = grid_size.height - 2;

    *origin = rotation_origin;
    origin.x += shift_x;
    origin.y += shift_y;

    for (x, y) in positions {
        let x = x + shift_x;
        let y = y + shift_y;

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
                GridPos { x, y },
                Block,
            ))
        ;
    }
}
