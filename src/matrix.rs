pub struct Matrix {
    pub width: i16,
    pub height: i16,
}

// Holds a block's position within a tetromino for rotation
#[derive(Debug, Clone, Copy)]
pub struct MatrixPosition {
    pub x: i16,
    pub y: i16,
}
