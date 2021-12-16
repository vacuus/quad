pub struct Matrix {
    pub width: i32,
    pub height: i32,
}

// Holds a block's position within a tetromino for rotation
#[derive(Debug, Clone, Copy)]
pub struct MatrixPosition {
    pub x: i32,
    pub y: i32,
}
