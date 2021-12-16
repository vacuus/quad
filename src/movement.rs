use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::rotation::rotate_tetromino;
use crate::heap::add_tetromino_to_heap;
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MoveTetrominoSystem;

pub struct SoftDropTimer(pub Timer);

pub struct MoveTetrominoTimer(pub Timer);

enum Direction {
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
    mut tetromino: Query<
        (Entity, &mut MatrixPosition), With<Tetromino>
    >,
    mut tetromino_type: ResMut<TetrominoType>,
) {

    // Each of the four blocks making up the current tetromino has,
    // appropriately, the 'Tetromino' component
    let (tetromino_ents, mut tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter_mut()
        .unzip()
    ;

    let prev_positions = tetromino_pos
        .iter()
        .map(|pos| **pos)
        .collect::<Vec<_>>()
    ;

    let matrix = matrix.single().unwrap();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        while can_move(&tetromino_pos, &matrix, Direction::Down, &*heap) {
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

    move_tetromino_timer.0.tick(time.delta());

    if move_tetromino_timer.0.just_finished() {
        move_tetromino_timer.0.reset();
    } else {
        // Ignore movement input, but soft drop still takes effect
        move_x = 0;
        move_y = 0;
    }

    // Check if moving left/right is legal
    if move_x == -1
        && !can_move(&tetromino_pos, &matrix, Direction::Left, &*heap)
    {
        move_x = 0;
    } else if move_x == 1
        && !can_move(&tetromino_pos, &matrix, Direction::Right, &*heap)
    {
        move_x = 0;
    }

    // Soft drop
    soft_drop_timer.0.tick(time.delta());

    if soft_drop_timer.0.just_finished() {
        move_y -= 1;
        soft_drop_timer.0.reset();
    }

    // Apply playing board bounds
    let mut x_offset = 0;

    tetromino_pos.iter_mut().for_each(|pos| {
        pos.x += move_x;

        if move_x == -1 {
            x_offset = x_offset.max(-pos.x);
        } else {
            x_offset = x_offset.min(matrix.width - pos.x - 1);
        }
    });

    tetromino_pos.iter_mut().for_each(|pos| pos.x += x_offset);

    let mut y_offset = 0;

    tetromino_pos.iter_mut().for_each(|pos| {
        pos.y += move_y;
        y_offset = y_offset.max(-pos.y);
    });

    tetromino_pos.iter_mut().for_each(|pos| pos.y += y_offset);


    let rotate_clockwise = if keyboard_input.just_pressed(KeyCode::X) {
        Some(true)
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        Some(false)
    } else {
        None
    };

    // Rotation
    if let Some(clockwise) = rotate_clockwise {
        use TetrominoType::*;

        let rotation_grid_size = match *tetromino_type {
            I | O => 4,
            T | Z | S | L | J => 3,
        };

        rotate_tetromino(
            &mut tetromino_pos,
            rotation_grid_size,
            &matrix,
            clockwise,
        );
    }

    if !can_move(&tetromino_pos, &matrix, Direction::Down, &heap) {
        if rotate_clockwise.is_some() {
            let mut should_revert = true;

            let try_moves = [
                (1, 0),
                (2, 0),
                (-1, 0),
                (-2, 0),
                (-1, -2), // T spins
                (1, -2),
            ];

            for try_move in &try_moves {
                tetromino_pos.iter_mut().for_each(|pos| {
                    pos.x += try_move.0;
                    pos.y += try_move.1;
                });

                if can_move(&tetromino_pos, &matrix, Direction::Down, &heap) {
                    should_revert = false;
                    break;
                }
            }

            if should_revert {
                tetromino_pos
                    .iter_mut()
                    .zip(&prev_positions)
                    .for_each(|(pos, prev_pos)| **pos = *prev_pos)
                ;
            }
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
    }
}

fn can_move(
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

            pos.y > 0 && match heap.get((x + y * matrix.width) as usize) {
                Some(None) => true,
                _ => false,
            }
        })
}
