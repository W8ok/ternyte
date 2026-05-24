use super::base::*;
use crate::sdl::types::*;

pub enum GateType {
    AND,
    OR,
    XOR,
    NOT,
    NAND,
    NOR,
    XNOR,
    BUF,
}
pub struct Placed;
pub struct Selected;
pub struct InputPoints(pub Vec<Position>);

#[derive(Clone)]
pub struct Moving {
    pub original_rect: Rect,
    pub original_points: Vec<Position>,
    pub offset: Position,
}
