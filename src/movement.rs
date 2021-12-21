use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::rotation::rotate_tetromino;
use crate::heap::add_tetromino_to_heap;
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};
use core::ops::{Deref, DerefMut};

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MoveTetrominoSystem;

macro_rules! timer {
    ($ty:ident) => {
        pub struct $ty(pub Timer);

        impl Deref for $ty {
            type Target = Timer;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    }
}

timer!(SoftDropTimer);
timer!(MoveTetrominoTimer);

pub enum Direction {
    Down,
    Left,
    Right,
}

pub fn move_tetromino(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    mut move_tetromino_timer: ResMut<MoveTetrominoTimer>,
    mut heap: ResMut<Vec<Option<()>>>,
    matrix: Query<&Matrix>,
    mut tetromino: Query<(Entity, &mut MatrixPosition), With<Tetromino>>,
    mut tetromino_type: ResMut<TetrominoType>,
) {
    // Each of the four blocks making up the current tetromino has,
    // appropriately, the 'Tetromino' component
    let (tetromino_ents, mut tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter_mut()
        .unzip()
    ;
    let matrix = matrix.single().unwrap();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        while can_move(&tetromino_pos, &matrix, Direction::Down, &heap) {
            tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
        }

        // Revert movement and add to heap
        add_tetromino_to_heap(
            &mut commands,
            &tetromino_ents,
            &mut heap,
            &tetromino_pos,
            &matrix,
        );
        spawn_tetromino(
            &mut commands,
            &matrix,
            &mut materials,
            &mut tetromino_type,
        );
        return;
    }


    let mut move_x = if keyboard_input.pressed(KeyCode::J)
        || keyboard_input.pressed(KeyCode::Left)
    {
        -1
    } else if keyboard_input.pressed(KeyCode::L)
        || keyboard_input.pressed(KeyCode::Right)
    {
        1
    } else {
        0
    };

    let mut move_y = if keyboard_input.pressed(KeyCode::K)
        || keyboard_input.pressed(KeyCode::Down)
    {
        -1
    } else {
        0
    };

    move_tetromino_timer.tick(time.delta());
    if !move_tetromino_timer.just_finished() {
        // Ignore movement input, but soft drop still takes effect
        move_x = 0;
        move_y = 0;
    }

    // Soft drop
    soft_drop_timer.tick(time.delta());
    if soft_drop_timer.just_finished() {
        move_y -= 1;
    }

    // Check if moving left/right is legal
    if move_x == -1
        && !can_move(&tetromino_pos, &matrix, Direction::Left, &heap)
    {
        move_x = 0;
    } else if move_x == 1
        && !can_move(&tetromino_pos, &matrix, Direction::Right, &*heap)
    {
        move_x = 0;
    }

    tetromino_pos.iter_mut().for_each(|pos| {
        pos.x += move_x;
        pos.y += move_y;
    });

    let rotate_clockwise = if keyboard_input.just_pressed(KeyCode::X) {
        Some(true)
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        Some(false)
    } else {
        None
    };

    // Rotation
    if let Some(clockwise) = rotate_clockwise {
        rotate_tetromino(
            &mut tetromino_pos,
            *tetromino_type,
            &matrix,
            &heap,
            clockwise,
        );
    }

    if !can_move(&tetromino_pos, &matrix, Direction::Down, &heap) {
        // Revert movement and add to heap
        add_tetromino_to_heap(
            &mut commands,
            &tetromino_ents,
            &mut heap,
            &tetromino_pos,
            &matrix,
        );
        spawn_tetromino(
            &mut commands,
            &matrix,
            &mut materials,
            &mut tetromino_type,
        );
    }
}

pub fn can_move(
    tetromino_pos: &Vec<Mut<MatrixPosition>>,
    matrix: &Matrix,
    direction: Direction,
    heap: &Vec<Option<()>>,
) -> bool {
    tetromino_pos
        .iter()
        .all(|pos| {
            use self::Direction::*;

            let (x, y) = match direction {
                Down => (pos.x, pos.y - 1),
                Left => (pos.x - 1, pos.y),
                Right => (pos.x + 1, pos.y),
            };
            let maybe_in_heap = match heap.get(
                (x + y * matrix.width) as usize
            ) {
                Some(None) => true,
                _ => false,
            };

            // invalid x or y will still likely produce a valid index into
            // 'heap'; the index is only accurate if x and y are in bounds
            x >= 0 && x < matrix.width && y >= 0 && maybe_in_heap
        })
}
