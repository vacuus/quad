use bevy::render::color::Color;
use crate::grid::GridPos;
use super::{Origin, OriginMode};


// starting positions
pub const I_POS: [(i16, i16); 4]  = [(0, 0), (1, 0), (2, 0), (3, 0)];
pub const I_ORIGIN: Origin = Origin {
    pos: GridPos { x: 2, y: 0 },
    mode: OriginMode::PointCentered,
};
pub const I_COLOR: Color = Color::rgb(0.0, 0.7, 0.7); // cyan

pub const O_POS: [(i16, i16); 4]  = [(0, 0), (0, 1), (1, 0), (1, 1)];
pub const O_ORIGIN: Origin = Origin {
    pos: GridPos { x: 1, y: 1 },
    mode: OriginMode::PointCentered,
};
pub const O_COLOR: Color = Color::rgb(0.7, 0.7, 0.0); // yellow

pub const T_POS: [(i16, i16); 4]  = [(0, 0), (1, 0), (2, 0), (1, 1)];
pub const T_ORIGIN: Origin = Origin {
    pos: GridPos { x: 1, y: 0 },
    mode: OriginMode::BlockCentered,
};
pub const T_COLOR: Color = Color::rgb(0.7, 0.0, 0.7); // purple

pub const Z_POS: [(i16, i16); 4]  = [(0, 1), (1, 1), (1, 0), (2, 0)];
pub const Z_ORIGIN: Origin = Origin {
    pos: GridPos { x: 1, y: 0 },
    mode: OriginMode::BlockCentered,
};
pub const Z_COLOR: Color = Color::rgb(0.7, 0.0, 0.0); // red

pub const S_POS: [(i16, i16); 4]  = [(2, 1), (1, 1), (1, 0), (0, 0)];
pub const S_ORIGIN: Origin = Origin {
    pos: GridPos { x: 1, y: 0 },
    mode: OriginMode::BlockCentered,
};
pub const S_COLOR: Color = Color::rgb(0.0, 0.7, 0.0); // green

pub const L_POS: [(i16, i16); 4]  = [(0, 0), (0, 1), (1, 0), (2, 0)];
pub const L_ORIGIN: Origin = Origin {
    pos: GridPos { x: 1, y: 0 },
    mode: OriginMode::BlockCentered,
};
pub const L_COLOR: Color = Color::rgb(0.0, 0.0, 0.9); // blue

pub const J_POS: [(i16, i16); 4]  = [(0, 0), (1, 0), (2, 0), (2, 1)];
pub const J_ORIGIN: Origin = Origin {
    pos: GridPos { x: 1, y: 0 },
    mode: OriginMode::BlockCentered,
};
pub const J_COLOR: Color = Color::rgb(0.9, 0.2, 0.0); // orange
