use bevy::prelude::*;
use crate::matrix::{Matrix, MatrixPosition};
use crate::rotation::rotate_tetromino;
use crate::heap::add_tetromino_to_heap;
use crate::tetromino::{Tetromino, TetrominoType, spawn_tetromino};
use core::ops::{Deref, DerefMut};

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSystemLabel;

// Newtype wrapper around a `Timer`
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

timer!(GravityTimer);
timer!(MovementTimer);
timer!(LockDelayTimer);

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    DownBy1,
    DownBy2,
    Left,
    Right,
    Neutral,
}

pub fn movement(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    mut movement_timer: ResMut<MovementTimer>,
    mut lock_delay_timer: ResMut<LockDelayTimer>,
    mut heap: ResMut<Vec<Option<()>>>,
    matrix: Query<&Matrix>,
    mut tetromino: Query<(Entity, &mut MatrixPosition), With<Tetromino>>,
    mut tetromino_type: ResMut<TetrominoType>,
) {
    // Each block of the tetromino has, appropriately, the `Tetromino` component
    let (tetromino_ents, mut tetromino_pos): (Vec<_>, Vec<_>) = tetromino
        .iter_mut()
        .unzip()
    ;
    let matrix = matrix.single().unwrap();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        hard_drop(
            &mut commands,
            &matrix,
            &tetromino_ents,
            &mut tetromino_pos,
            &mut heap,
            &mut materials,
            &mut tetromino_type,
        );
        return;
    }

    use self::Direction::*;

    // Get movement input
    let mut move_x = if keyboard_input.pressed(KeyCode::J)
        || keyboard_input.pressed(KeyCode::Left)
    {
        Left
    } else if keyboard_input.pressed(KeyCode::L)
        || keyboard_input.pressed(KeyCode::Right)
    {
        Right
    } else {
        Neutral
    };
    let mut move_y = if keyboard_input.pressed(KeyCode::K)
        || keyboard_input.pressed(KeyCode::Down)
    {
        DownBy1
    } else {
        Neutral
    };

    // Only allow movement every so often
    movement_timer.tick(time.delta());
    if !movement_timer.just_finished() {
        // Ignore movement input, but gravity can still take effect
        move_x = Neutral;
        move_y = Neutral;
    } else {
        movement_timer.reset();
    }

    // Gravity
    gravity_timer.tick(time.delta());
    if gravity_timer.just_finished() {
        move_y = match move_y {
            Neutral => DownBy1,
            // Though unlikely, the user and the soft drop could each
            // decrement 'move_y' on the same frame
            DownBy1 => DownBy2,
            _ => unreachable!(),
        };
        gravity_timer.reset();
    }

    // Check if movement is legal
    if !can_move(&tetromino_pos, &matrix, move_x, &heap) {
        move_x = Neutral;
    }
    if !can_move(&tetromino_pos, &matrix, move_y, &heap) {
        move_y = match move_y {
            DownBy1 => Neutral,
            DownBy2 => if !can_move(&tetromino_pos, &matrix, DownBy1, &heap) {
                Neutral
            } else {
                DownBy1
            },
            _ => unreachable!(),
        }
    }

    // Apply movement
    tetromino_pos.iter_mut().for_each(|pos| {
        pos.x += match move_x {
            Neutral => 0,
            Left => -1,
            Right => 1,
            _ => unreachable!(),
        };
        pos.y += match move_y {
            Neutral => 0,
            DownBy1 => -1,
            DownBy2 => -2,
            _ => unreachable!(),
        };
    });

    // Get rotation input
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

    // Reset lock delay if any input
    if move_x != Neutral || move_y != Neutral || rotate_clockwise.is_some() {
        lock_delay_timer.reset();
    }
    if !can_move(&tetromino_pos, &matrix, Direction::DownBy1, &heap) {
        // If the tetromino can't move down, commence/continue the lock delay
        lock_delay_timer.tick(time.delta());
        if !lock_delay_timer.just_finished() {
            return;
        }
        lock_delay_timer.reset();
        // Revert movement and add tetromino to heap
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

            // Get neighboring position in relevant direction
            let (x, y) = match direction {
                DownBy1 => (pos.x, pos.y - 1),
                DownBy2 => (pos.x, pos.y - 2),
                Left => (pos.x - 1, pos.y),
                Right => (pos.x + 1, pos.y),
                Neutral => return true,
            };
            // Check if the neighboring position is occupied in the heap
            let maybe_in_heap = match heap.get(
                (x + y * matrix.width) as usize
            ) {
                Some(None) => true,
                _ => false,
            };

            // Invalid x or y will still likely produce a valid index into
            // 'heap'; the index is only accurate if x and y are in bounds
            x >= 0 && x < matrix.width && y >= 0 && maybe_in_heap
        })
}

fn hard_drop(
    mut commands: &mut Commands,
    matrix: &Matrix,
    tetromino_ents: &Vec<Entity>,
    tetromino_pos: &mut Vec<Mut<MatrixPosition>>,
    mut heap: &mut Vec<Option<()>>,
    mut materials: &mut Assets<ColorMaterial>,
    mut tetromino_type: &mut TetrominoType,
) {
    while can_move(&tetromino_pos, &matrix, Direction::DownBy1, &heap) {
        tetromino_pos.iter_mut().for_each(|pos| pos.y -= 1);
    }

    // Revert movement and add tetromino to heap
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
