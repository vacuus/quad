use ::core::ops::{Deref, DerefMut};
use bevy::core::Timer;
use bevy::ecs::schedule::SystemLabel;

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq, PartialOrd)]
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
pub enum Move {
    X(X),
    Y(Y),
}

impl Move {
    pub fn move_down(&mut self) {
        *self = match self {
            Self::Y(Y::Neutral) => Self::Y(Y::DownBy1),
            // Though unlikely, the user and the soft drop could
            // each decrement 'move_y' on the same frame
            Self::Y(Y::DownBy1) => Self::Y(Y::DownBy2),
            _ => *self,
        }
    }

    pub fn move_up(&mut self) {
        *self = match self {
            Self::Y(Y::DownBy1) => Self::Y(Y::Neutral),
            Self::Y(Y::DownBy2) => Self::Y(Y::DownBy1),
            _ => *self,
        }
    }

    pub fn set_neutral(&mut self) {
        *self = match self {
            Self::X(_) => Self::X(X::Neutral),
            Self::Y(_) => Self::Y(Y::Neutral),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum X {
    Left,
    Right,
    Neutral,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Y {
    DownBy1,
    DownBy2,
    HardDrop,
    Neutral,
}
