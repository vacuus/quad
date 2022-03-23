use bevy::core::Timer;
use ::core::ops::{Deref, DerefMut};


pub struct ResetLockDelay(bool);

impl ResetLockDelay {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn set_to(&mut self, state: bool) {
        self.0 = state;
    }

    pub fn get(&self) -> bool {
        self.0
    }
}

pub struct HardDropOccurred(bool);

impl HardDropOccurred {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn set(&mut self) {
        self.0 = true;
    }

    pub fn reset(&mut self) {
        self.0 = false;
    }

    pub fn get(&self) -> bool {
        self.0
    }
}

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

pub trait MoveOffset {
    fn set_neutral(&mut self);

    fn is_neutral(&self) -> bool;
}

#[derive(Copy, Clone, PartialEq)]
pub struct MoveNeutral;

impl MoveOffset for MoveNeutral {
    fn set_neutral(&mut self) {
        *self = Self;
    }

    fn is_neutral(&self) -> bool {
        true
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum MoveX {
    Left,
    Right,
    Neutral,
}

impl MoveOffset for MoveX {
    fn set_neutral(&mut self) {
        *self = Self::Neutral;
    }

    fn is_neutral(&self) -> bool {
        *self == Self::Neutral
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
            // Though unlikely, the user and the soft drop could
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
    fn set_neutral(&mut self) {
        *self = Self::Neutral;
    }

    fn is_neutral(&self) -> bool {
        *self == Self::Neutral
    }
}
