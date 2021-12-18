use core::ops::AddAssign;

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

impl AddAssign<(i16, i16)> for MatrixPosition {
    fn add_assign(&mut self, rhs: (i16, i16)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}
