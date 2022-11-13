use bevy::ecs::{
    world::Mut,
    component::Component,
    system::Resource,
};
use ::core::ops::{Add, AddAssign};
use ::core::borrow::Borrow;


#[derive(Resource)]
pub struct GridSize {
    pub width: i16,
    pub height: i16,
}

// Holds a block's position within a piece for rotation
#[derive(Debug, Clone, Copy, Component)]
pub struct GridPos {
    pub x: i16,
    pub y: i16,
}

impl Add<(i16, i16)> for GridPos {
    type Output = GridPos;

    fn add(self, rhs: (i16, i16)) -> GridPos {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl AddAssign<(i16, i16)> for GridPos {
    fn add_assign(&mut self, rhs: (i16, i16)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl Borrow<GridPos> for &Mut<'_, GridPos> {
    fn borrow(&self) -> &GridPos {
        &*self
    }
}

impl Borrow<GridPos> for &&GridPos {
    fn borrow(&self) -> &GridPos {
        *self
    }
}
