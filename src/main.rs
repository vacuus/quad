use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use ::std::collections::BTreeSet;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(SoftDropTimer(Timer::from_seconds(0.750, true)))
        .insert_resource(PrintInfoTimer(Timer::from_seconds(1.0, true)))
        .insert_resource(BTreeSet::<MatrixPosition>::new())
        .add_startup_system(setup.system())
        .add_system(print_info.system())
        .add_system(update_block_sprites.system())
        .add_system(move_current_tetromino.system())
        .run();
}

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

// A block can be part of the heap.
struct Heap;

const BLOCK_SIZE: f32 = 25.0;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let matrix = Matrix {
        width: 10,
        height: 22,
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    spawn_current_tetromino(&mut commands, &matrix, &mut materials);

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
    curr_tetromino_query: Query<&MatrixPosition, With<Tetromino>>
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        eprintln!("Positions of blocks in current tetromino:");
        curr_tetromino_query
            .iter()
            .inspect(|pos| eprintln!("{:?}", pos))
            .last();
        timer.0.reset();
    }
}

fn move_current_tetromino(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut soft_drop_timer: ResMut<SoftDropTimer>,
    mut heap: ResMut<BTreeSet<MatrixPosition>>,
    matrix_query: Query<&Matrix>,
    mut curr_tetromino_query: Query<
        (Entity, &mut MatrixPosition), With<Tetromino>
    >,
    // The current tetromino isn't part of the heap anyways, but Bevy doesn't
    // know that; besides, it's fragile if the code is changed
    heap_query: Query<&MatrixPosition,
        (Added<Heap>, Without<Tetromino>),
    >,
) {
    fn can_move(
        curr_tetromino_pos: MatrixPosition,
        heap: &BTreeSet<MatrixPosition>,
    ) -> bool {
        curr_tetromino_pos.y >= 0 && !heap.contains(&curr_tetromino_pos)
    }

    // Each of the four blocks making up the current tetromino have,
    // appropriately, the 'Tetromino' component
    let curr_tetromino_blocks = curr_tetromino_query
        .iter_mut()
        .collect::<Vec<_>>();
    let position = &mut *position;
    let prev_position = (position.x, position.y);
    let matrix = matrix_query.single().unwrap();

    // 'heap' will only be updated when necessary
    for pos in heap_query.iter() {
        heap.insert(*pos);
    }

    // Hard drop
    if keyboard_input.just_pressed(KeyCode::I)
        || keyboard_input.just_pressed(KeyCode::Up)
    {
        while can_move(*position, &heap) {
            position.y -= 1;
        }

        position.y += 1;
        commands
            .entity(entity)
            .remove::<Tetromino>()
            .insert(Heap);

        spawn_current_tetromino(&mut commands, matrix, &mut materials);
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
    } else { 0 };

    let mut move_y = if keyboard_input.just_pressed(KeyCode::K)
        || keyboard_input.just_pressed(KeyCode::Down)
    {
        -1
    } else { 0 };

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

        let matrix_size =
            Tetromino::SIZES[curr_tetromino.tetromino_type as usize];
        rotate_tetromino_block(&mut curr_tetromino, matrix_size, clockwise);

        move_x += curr_tetromino.index.x - prev_index_x;
        move_y += curr_tetromino.index.y - prev_index_y;
    }

    // Bounds
    if position.x + move_x < 0 {
        x_over = (position.x + move_x).min(x_over);
    } else if position.x + move_x >= matrix.width {
        x_over = ((position.x + move_x) - matrix.width + 1).max(x_over);
    }

    position.x += move_x;
    position.x -= x_over;

    position.y += move_y;
    position.y -= y_over;

    // TODO: Probably better off setting the matrix up so you can index into
    // it to look for occupied spots around the current tetromino
    if !can_move(*position, &heap) {
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
                position.x += try_move.0;
                position.y += try_move.1;

                if can_move(*position, &heap) {
                    should_revert = false;
                    break;
                }
            }
        } else {
            // Revert movement and add to heap
            commands
                .entity(entity)
                .remove::<Tetromino>()
                .insert(Heap);

            spawn_current_tetromino(&mut commands, matrix, &mut materials);
        }

        if should_revert {
            position.x = prev_position.0;
            position.y = prev_position.1;
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
            (position.x as f32 - matrix.width as f32 * 0.5 + 0.5);
        let new_y = BLOCK_SIZE *
            (position.y as f32 - matrix.height as f32 * 0.5 + 0.5);

        *transform = Transform::from_xyz(new_x, new_y, transform.translation.z);
    }
}

// ----------------
// UTILITY AND IMPL
// ----------------

fn rotate_tetromino_block(
    tetromino_block: &mut Tetromino,
    matrix_size: i32,
    clockwise: bool,
) {
    let orig_x = tetromino_block.index.x;
    let orig_y = tetromino_block.index.y;
    let matrix_size = matrix_size - 1;

    let x = orig_x;
    if clockwise {
        tetromino_block.index.x = orig_y;
        tetromino_block.index.y = matrix_size - x;
    } else {
        tetromino_block.index.x = matrix_size - orig_y;
        tetromino_block.index.y = orig_x;
    }
}

fn spawn_current_tetromino(
    commands: &mut Commands,
    matrix: &Matrix,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for (color, position) in Tetromino::blocks_from_type(rand::random()) {
        let tetromino_matrix_size =
            Tetromino::SIZES[block.tetromino_type as usize];

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                sprite: Sprite::new(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                transform: Transform::from_translation(
                    Vec3::new(0.0, 0.0, 1.0)
                ),
                ..Default::default()
            })
            .insert(MatrixPosition {
                x: position.x + 3,
                y: matrix.height - tetromino_matrix_size + position.y,
            })
            .insert(Tetromino)
        ;
    }
}

#[derive(Copy, Clone, Debug)]
enum TetrominoType {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    L = 5,
    J = 6,
}

impl Tetromino {
    const BLOCK_INDICES: [[(i32, i32); 4]; 7] = [
        [
            // line, cyan
            (1, 3),
            (1, 2),
            (1, 1),
            (1, 0),
        ],
        [
            // square, yellow
            (1, 1),
            (1, 2),
            (2, 1),
            (2, 2),
        ],
        [
            // T, purple
            (0, 1),
            (1, 1),
            (2, 1),
            (1, 2),
        ],
        [
            // Z, red
            (0, 2),
            (1, 2),
            (1, 1),
            (2, 1),
        ],
        [
            // S, green
            (2, 2),
            (1, 2),
            (1, 1),
            (0, 1),
        ],
        [
            // L, blue
            (0, 2),
            (0, 1),
            (1, 1),
            (2, 1),
        ],
        [
            // J, orange
            (0, 1),
            (1, 1),
            (2, 1),
            (2, 2),
        ],
    ];

    const COLORS: [(f32, f32, f32); 7] = [
        (0.0, 0.7, 0.7),  // line, cyan
        (0.7, 0.7, 0.0),  // square, yellow
        (0.7, 0.0, 0.7),  // T, purple
        (0.7, 0.0, 0.0),  // Z, red
        (0.0, 0.7, 0.0),  // S, green
        (0.0, 0.0, 0.7),  // L, blue
        (0.9, 0.25, 0.0), // J, orange
    ];

    const SIZES: [i32; 7] = [
        4, // line, cyan
        4, // square, yellow
        3, // T, purple
        3, // Z, red
        3, // S, green
        3, // L, blue
        3, // J, orange
    ];

    fn blocks_from_type(tetromino_type: TetrominoType)
    -> impl Iterator<Item = (Color, MatrixPosition)> {
        let type_usize = tetromino_type as usize;
        let color = Tetromino::COLORS[type_usize];

        Tetromino::BLOCK_INDICES[type_usize]
            .iter()
            .map(move |index| {
                (
                    Color::rgb(color.0, color.1, color.2),
                    MatrixPosition {
                        x: index.0,
                        y: index.1,
                    },
                )
            })
    }
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        match rng.gen_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::S,
            4 => TetrominoType::Z,
            5 => TetrominoType::L,
            _ => TetrominoType::J,
        }
    }
}
