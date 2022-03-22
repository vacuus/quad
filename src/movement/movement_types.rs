use bevy::core::Timer;
use ::core::ops::{Deref, DerefMut};


// Newtype wrapper for certain state
macro_rules! make_state {
    ($ty:ident) => {
        pub struct $ty(bool);

        impl $ty {
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
    }
}

make_state!(ResetLockDelay);
make_state!(HardDropOccurred);

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
    Neutral,
}

impl Move {
    pub fn move_down(&mut self) {
        *self = match self {
            Self::Neutral => Self::Y(Y::DownBy1),
            // Though unlikely, the user and the soft drop could
            // each decrement 'move_y' on the same frame
            Self::Y(Y::DownBy1) => Self::Y(Y::DownBy2),
            _ => *self,
        }
    }

    pub fn move_up(&mut self) {
        *self = match self {
            Self::Y(Y::DownBy1) => Self::Neutral,
            Self::Y(Y::DownBy2) => Self::Y(Y::DownBy1),
            _ => *self,
        }
    }

    pub fn set_neutral(&mut self) {
        *self = Self::Neutral;
    }

    pub fn is_neutral(&self) -> bool {
        *self == Self::Neutral
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum X {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Y {
    DownBy1,
    DownBy2,
}
