mod movement;
mod matrix;
mod tetromino;
mod rotation;
mod heap;
mod processing;
mod kb_input;

use bevy::prelude::*;
use movement::{
    GravityTimer,
    MovementXTimer,
    MovementYTimer,
    movement,
};
use rotation::rotation;
use matrix::{Matrix, MatrixPosition};
use tetromino::spawn_tetromino;
use heap::HeapEntry;
use processing::processing;
use kb_input::{KeyActions, keyboard_input};


// pixel (?) width of a block
const BLOCK_SIZE: f32 = 25.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GravityTimer::new())
        .insert_resource(MovementXTimer::new())
        .insert_resource(MovementYTimer::new())
        .insert_resource(KeyActions::new())
        .insert_resource(MatrixPosition { x: 0, y: 0})
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(movement.after(keyboard_input))
        .add_system(rotation.after(movement))
        .add_system(processing.after(rotation))
        .add_system(update_sprites.after(processing))
        .run()
    ;
}

fn setup(mut commands: Commands, mut origin: ResMut<MatrixPosition>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let matrix = Matrix {
        width: 15,
        height: 25,
    };

    commands.insert_resource(
        vec![HeapEntry::Vacant; (matrix.width * matrix.height) as usize],
    );

    spawn_tetromino(&mut commands, &matrix, &mut origin);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    matrix.width as f32 * BLOCK_SIZE,
                    matrix.height as f32 * BLOCK_SIZE,
                )),
                color: Color::rgb(0.0, 0.0, 0.0),
                ..Sprite::default()
            },
            ..SpriteBundle::default()
        })
        .insert(matrix)
    ;
}

fn update_sprites(
    matrix: Query<&Matrix>,
    mut block: Query<(&MatrixPosition, &mut Transform)>,
) {
    let matrix = matrix.single();

    for (position, mut transform) in block.iter_mut() {
        transform.translation.x = BLOCK_SIZE *
            (position.x as f32 - matrix.width as f32 * 0.5 + 0.5)
        ;
        transform.translation.y = BLOCK_SIZE *
            (position.y as f32 - matrix.height as f32 * 0.5 + 0.5)
        ;
    }
}
