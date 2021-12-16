mod movement;
mod matrix;
mod tetromino;
mod rotation;
mod heap;

use bevy::prelude::*;
use movement::{
    move_tetromino,
    MoveTetrominoSystem,
    SoftDropTimer,
    MoveTetrominoTimer,
};
use matrix::{Matrix, MatrixPosition};
use tetromino::{TetrominoType, spawn_tetromino};


const BLOCK_SIZE: f32 = 25.0;


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(SoftDropTimer(Timer::from_seconds(0.750, true)))
        .insert_resource(MoveTetrominoTimer(Timer::from_seconds(0.0625, true)))
        .insert_resource(Vec::<Option<()>>::new()) // just a placeholder
        .insert_resource(rand::random::<TetrominoType>()) // also a placeholder
        .add_startup_system(setup.system())
        .add_system(move_tetromino.system().label(MoveTetrominoSystem))
        .add_system(update_block_sprites.system().after(MoveTetrominoSystem))
        .run()
    ;
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut tetromino_type: ResMut<TetrominoType>,
    mut heap: ResMut<Vec<Option<()>>>,
) {
    let matrix = Matrix {
        width: 10,
        height: 22,
    };

    *heap = vec![None; (matrix.width * matrix.height) as usize];

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_tetromino(
        &mut commands,
        &matrix,
        &mut materials,
        &mut tetromino_type,
    );

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            sprite: Sprite::new(Vec2::new(
                matrix.width as f32 * BLOCK_SIZE,
                matrix.height as f32 * BLOCK_SIZE,
            )),
            ..Default::default()
        })
        .insert(matrix)
    ;
}

fn update_block_sprites(
    matrix: Query<&Matrix>,
    mut block: Query<(&MatrixPosition, &mut Transform)>,
) {
    let matrix = matrix.single().unwrap();

    for (position, mut transform) in block.iter_mut() {
        transform.translation.x = BLOCK_SIZE *
            (position.x as f32 - matrix.width as f32 * 0.5 + 0.5)
        ;
        transform.translation.y = BLOCK_SIZE *
            (position.y as f32 - matrix.height as f32 * 0.5 + 0.5)
        ;
    }
}
