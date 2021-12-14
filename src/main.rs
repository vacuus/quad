use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};


const BLOCK_SIZE: f32 = 25.0;


struct SoftDropTimer(Timer);

struct PrintInfoTimer(Timer);

struct Matrix {
    width: i32,
    height: i32,
}

// Holds a block's position within a tetromino for rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MatrixPosition {
    x: i32,
    y: i32,
}

// A block can be part of the current tetromino
#[derive(Debug)]
struct Tetromino;

impl Tetromino {
    fn blocks_from_type(tetromino_type: TetrominoType)
    -> (i32, Color, [(i32, i32); 4]) {
        use self::TetrominoType::*;
    
        let matrix_size = match tetromino_type {
            I | O => 4,
            T | Z | S | L | J => 3,
        };
    
        let color = match tetromino_type {
            I => (0.0, 0.7, 0.7),  // cyan
            O => (0.7, 0.7, 0.0),  // yellow
            T => (0.7, 0.0, 0.7),  // purple
            Z => (0.7, 0.0, 0.0),  // red
            S => (0.0, 0.7, 0.0),  // green
            L => (0.0, 0.0, 0.7),  // blue
            J => (0.9, 0.25, 0.0), // orange
        };

        let color = Color::rgb(color.0, color.1, color.2);

        let positions = match tetromino_type {
            I => [(1, 3), (1, 2), (1, 1), (1, 0)],
            O => [(1, 1), (1, 2), (2, 1), (2, 2)],
            T => [(0, 1), (1, 1), (2, 1), (1, 2)],
            Z => [(0, 2), (1, 2), (1, 1), (2, 1)],
            S => [(2, 2), (1, 2), (1, 1), (0, 1)],
            L => [(0, 2), (0, 1), (1, 1), (2, 1)],
            J => [(0, 1), (1, 1), (2, 1), (2, 2)],
        };

        (matrix_size, color, positions)
    }
}

#[derive(Copy, Clone, Debug)]
enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        use self::TetrominoType::*;

        match rng.gen_range(0..7) {
            0 => I,
            1 => O,
            2 => T,
            3 => S,
            4 => Z,
            5 => L,
            6 => J,
        }
    }
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(SoftDropTimer(Timer::from_seconds(0.750, true)))
        .insert_resource(PrintInfoTimer(Timer::from_seconds(1.0, true)))
        .insert_resource(Vec::<Option<()>>::new()) // just a placeholder
        .insert_resource(rand::random::<TetrominoType>()) // also a placeholder
        .add_startup_system(setup.system())
        .add_system(print_info.system())
        .add_system(update_block_sprites.system())
        .add_system(move_current_tetromino.system())
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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_current_tetromino(
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

fn print_info(
    time: Res<Time>,
    mut timer: ResMut<PrintInfoTimer>,
    tetromino_query: Query<&MatrixPosition, With<Tetromino>>
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        eprintln!("Positions of blocks in current tetromino:");

        tetromino_query
            .iter()
            .for_each(|pos| eprintln!("{:?}", pos))
        ;

        timer.0.reset();
    }
}

fn move_current_tetromino(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    mut heap: ResMut<Vec<Option<()>>>,
    matrix_query: Query<&Matrix>,
    mut tetromino_query: Query<
        (Entity, &mut MatrixPosition), With<Tetromino>
    >,
    mut tetromino_type: ResMut<TetrominoType>,
) {
    fn can_move(
        tetromino_pos: &Vec<Mut<MatrixPosition>>,
        matrix: &Matrix,
        heap: &Vec<Option<()>>,
    ) -> bool {
        tetromino_pos
            .iter()
            .all(|pos| pos.y >= 0
                && heap.get((pos.x + pos.y * matrix.width) as usize).is_none()
            )
    }

    // Each of the four blocks making up the current tetromino has,
    // appropriately, the 'Tetromino' component
    let (tetromino_ents, tetromino_pos): (Vec<_>, Vec<_>) = tetromino_query
        .iter_mut()
        .unzip()
    ;

    let prev_positions = tetromino_pos
        .iter()
        .map(|pos| **pos)
        .collect::<Vec<_>>()
    ;

    let matrix = matrix_query.single().unwrap();

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        while can_move(&tetromino_pos, &matrix, &*heap) {
            tetromino_pos.iter().for_each(|pos| pos.y -= 1);
        }

        tetromino_pos.iter().for_each(|pos| pos.y += 1);

        // Revert movement and add to heap
        add_tetromino_to_heap(
            &commands,
            &tetromino_ents,
            &mut heap,
            &tetromino_pos,
            &matrix,
        );

        spawn_current_tetromino(
            &mut commands,
            matrix,
            &mut materials,
            &mut tetromino_type,
        );

        return;
    }

    let mut move_x = if keyboard_input.just_pressed(KeyCode::J)
        || keyboard_input.just_pressed(KeyCode::Left)
    {
        -1
    } else if keyboard_input.just_pressed(KeyCode::L)
        || keyboard_input.just_pressed(KeyCode::Right)
    {
        1
    } else {
        0
    };

    let mut move_y = if keyboard_input.just_pressed(KeyCode::K)
        || keyboard_input.just_pressed(KeyCode::Down)
    {
        -1
    } else {
        0
    };

    // Movement
    soft_drop_timer.0.tick(time.delta());

    if soft_drop_timer.0.just_finished() {
        move_y -= 1;
        soft_drop_timer.0.reset();
    }

    let rotate_clockwise = if keyboard_input.just_pressed(KeyCode::X) {
        Some(true)
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        Some(false)
    } else {
        None
    };

    let mut x_over = 0;
    let mut y_over = 0;

    // Rotation
    if let Some(clockwise) = rotate_clockwise {
        let prev_index_x = curr_tetromino.index.x;
        let prev_index_y = curr_tetromino.index.y;

        rotate_tetromino_block(&tetromino_pos, matrix_size, clockwise);

        move_x += curr_tetromino.index.x - prev_index_x;
        move_y += curr_tetromino.index.y - prev_index_y;
    }

    // Bounds
    if position.x + move_x < 0 {
        x_over = (position.x + move_x).min(x_over);
    } else if position.x + move_x >= matrix.width {
        x_over = ((position.x + move_x) - matrix.width + 1).max(x_over);
    }

    tetromino_pos.iter().for_each(|pos| {
        pos.x += move_x;
        pos.x -= x_over;
    
        pos.y += move_y;
        pos.y -= y_over;
    });

    // TODO: Probably better off setting the matrix up so you can index into
    // it to look for occupied spots around the current tetromino
    if !can_move(&tetromino_pos, &matrix, &heap) {
        let mut should_revert = true;

        if let Some(_) = rotate_clockwise {
            let try_moves = [
                (1, 0),
                (2, 0),
                (-1, 0),
                (-2, 0),
                (-1, -2), // T spins
                (1, -2),
            ];

            for try_move in try_moves.iter() {
                tetromino_pos.iter().for_each(|pos| {
                    pos.x += try_move.0;
                    pos.y += try_move.1;
                });

                if can_move(&tetromino_pos, &matrix, &heap) {
                    should_revert = false;
                    break;
                }
            }
        } else {
            // Revert movement and add to heap
            add_tetromino_to_heap(
                &commands,
                &tetromino_ents,
                &mut heap,
                &tetromino_pos,
                &matrix,
            );

            spawn_current_tetromino(
                &mut commands,
                matrix,
                &mut materials,
                &mut tetromino_type,
            );
        }

        if should_revert {
            tetromino_pos
                .iter()
                .zip(&prev_positions)
                .for_each(|(pos, prev_pos)| **pos = *prev_pos)
            ;
        }
    }
}

fn update_block_sprites(
    matrix_query: Query<&Matrix>,
    mut block_query: Query<(&MatrixPosition, &mut Transform)>,
) {
    let matrix = matrix_query.single().unwrap();

    for (position, mut transform) in block_query.iter_mut() {
        let new_x = BLOCK_SIZE * 
            (position.x as f32 - matrix.width as f32 * 0.5 + 0.5)
        ;
        let new_y = BLOCK_SIZE *
            (position.y as f32 - matrix.height as f32 * 0.5 + 0.5)
        ;

        *transform = Transform::from_xyz(new_x, new_y, transform.translation.z);
    }
}

// ----------------
// UTILITY AND IMPL
// ----------------

fn add_tetromino_to_heap(
    commands: &Commands,
    tetromino_ents: &Vec<Entity>,
    heap: &mut ResMut<Vec<Option<()>>>,
    tetromino_pos: &Vec<Mut<MatrixPosition>>,
    matrix: &Matrix,
) {
    tetromino_ents
        .iter()
        .for_each(|&entity| {
            commands
                .entity(entity)
                .remove::<Tetromino>()
            ;
        })
    ;

    tetromino_pos
        .iter()
        .for_each(|pos| {
            heap[(pos.x + pos.y * matrix.width) as usize] = Some(());
        })
    ;
}

fn rotate_tetromino_block(
    tetromino_pos: &Vec<Mut<MatrixPosition>>,
    matrix_size: i32,
    clockwise: bool,
) {
    for pos in tetromino_pos {
        let orig_x = pos.x;
        let orig_y = pos.y;
        let matrix_size = matrix_size - 1;
    
        let x = orig_x;
        if clockwise {
            pos.x = orig_y;
            pos.y = matrix_size - x;
        } else {
            pos.x = matrix_size - orig_y;
            pos.y = orig_x;
        }
    }
}

fn spawn_current_tetromino(
    commands: &mut Commands,
    matrix: &Matrix,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    tetromino_type: &mut ResMut<TetrominoType>,
) {
    **tetromino_type = rand::random::<TetrominoType>();

    let (tetromino_matrix_size, color, positions) = Tetromino::blocks_from_type(
        **tetromino_type
    );

    for (x, y) in positions {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                sprite: Sprite::new(Vec2::splat(BLOCK_SIZE)),
                transform: Transform::from_translation(
                    Vec3::new(0.0, 0.0, 1.0)
                ),
                ..Default::default()
            })
            .insert(MatrixPosition {
                x: x + 3,
                y: matrix.height - tetromino_matrix_size + y,
            })
            .insert(Tetromino)
        ;
    }
}
