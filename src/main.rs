mod movement;
mod grid;
mod tetromino;
mod rotation;
mod heap;
mod input;

use bevy::prelude::*;
use movement::{
    GravityTimer,
    MovementXTimer,
    MovementYTimer,
    movement,
};
use rotation::rotation;
use grid::{GridSize, GridPos};
use tetromino::{SpawnEvent, spawn};
use heap::{HeapEntry, lock};
use input::{KeyActions, input};


// pixel (?) width of a block
const BLOCK_SIZE: f32 = 25.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GravityTimer::new())
        .insert_resource(MovementXTimer::new())
        .insert_resource(MovementYTimer::new())
        .insert_resource(KeyActions::new())
        // make this extensible
        .insert_resource(GridSize { width: 10, height: 25 })
        .insert_resource(GridPos { x: 0, y: 0})
        // placeholder for max_y
        .insert_resource(0_i16)
        .add_event::<SpawnEvent>()
        .add_startup_system(setup)
        .add_system(spawn)
        .add_system(input.after(spawn))
        .add_system(movement.after(input))
        .add_system(rotation.after(movement))
        .add_system(lock.after(rotation))
        .add_system(update_sprites.after(lock))
        .run()
    ;
}

fn setup(
    mut commands: Commands,
    grid_size: Res<GridSize>,
    mut spawn_notify: EventWriter<SpawnEvent>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.insert_resource(
        vec![HeapEntry::Vacant; (grid_size.width * grid_size.height) as usize],
    );

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    grid_size.width as f32 * BLOCK_SIZE,
                    grid_size.height as f32 * BLOCK_SIZE,
                )),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        })
    ;

    spawn_notify.send(SpawnEvent);
}

fn update_sprites(
    grid_size: Res<GridSize>,
    mut block: Query<(&GridPos, &mut Transform)>,
) {
    for (position, mut transform) in block.iter_mut() {
        transform.translation.x = BLOCK_SIZE *
            (position.x as f32 - grid_size.width as f32 * 0.5 + 0.5)
        ;
        transform.translation.y = BLOCK_SIZE *
            (position.y as f32 - grid_size.height as f32 * 0.5 + 0.5)
        ;
    }
}
