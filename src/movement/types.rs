use bevy::time::{Timer, TimerMode};
use bevy::prelude::{Deref, DerefMut, Resource};


// Newtype wrapper around a `Timer`
macro_rules! timer {
    ($ty:ident, $duration:literal) => {
        #[derive(Deref, DerefMut, Resource)]
        pub struct $ty(Timer);

        impl $ty {
            pub fn new() -> Self {
                Self(Timer::from_seconds($duration, TimerMode::Once))
            }
        }
    }
}

timer!(GravityTimer, 0.75);
timer!(MovementXTimer, 0.08);
timer!(MovementYTimer, 0.08);

pub trait MoveOffset: PartialEq + Sized {
    const NEUTRAL: Self;

    fn set_neutral(&mut self) {
        *self = <Self as MoveOffset>::NEUTRAL;
    }

    fn is_neutral(&self) -> bool {
        *self == <Self as MoveOffset>::NEUTRAL
    }

    fn to_offset(&self) -> (i16, i16);
}

#[derive(Copy, Clone, PartialEq)]
pub struct MoveNeutral;

impl MoveOffset for MoveNeutral {
    const NEUTRAL: Self = Self;

    fn to_offset(&self) -> (i16, i16) {
        (0, 0)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum MoveX {
    Left,
    Right,
    Neutral,
}

impl MoveOffset for MoveX {
    const NEUTRAL: Self = Self::Neutral;

    fn to_offset(&self) -> (i16, i16) {
        match *self {
            Self::Neutral => (0, 0),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum MoveY {
    Down1,
    Down2,
    Neutral,
}

impl MoveY {
    pub fn move_down(&mut self) {
        *self = match self {
            Self::Neutral => Self::Down1,
            // Though unlikely, the user and the gravity could
            // each decrement 'move_y' on the same frame
            Self::Down1 => Self::Down2,
            _ => *self,
        }
    }

    pub fn move_up(&mut self) {
        *self = match self {
            Self::Down1 => Self::Neutral,
            Self::Down2 => Self::Down1,
            _ => *self,
        }
    }
}

impl MoveOffset for MoveY {
    const NEUTRAL: Self = Self::Neutral;

    fn to_offset(&self) -> (i16, i16) {
        match *self {
            Self::Neutral => (0, 0),
            Self::Down1 => (0, -1),
            Self::Down2 => (0, -2),
        }
    }
}

impl MoveOffset for (MoveX, MoveY) {
    const NEUTRAL: Self = (MoveX::Neutral, MoveY::Neutral);

    fn to_offset(&self) -> (i16, i16) {
        (self.0.to_offset().0, self.1.to_offset().1)
    }
}
