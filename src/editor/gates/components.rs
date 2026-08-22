use super::super::components::*;
use crate::components::*;

#[derive(Debug, Clone, Copy, Default)]
pub enum GateKind {
    #[default]
    UNKNOWN,

    // Binary
    AND(ConnectionPoints<2, 1>),
    NAND(ConnectionPoints<2, 1>),
    OR(ConnectionPoints<2, 1>),
    NOR(ConnectionPoints<2, 1>),
    NOT(ConnectionPoints<1, 1>),
    BUF(ConnectionPoints<1, 1>),
    XOR(ConnectionPoints<2, 1>),
    XNOR(ConnectionPoints<2, 1>),
    // Ternary
}

pub struct Gate {
    pub kind: GateKind,
    pub rect: Rect,
}

pub struct ConnectionPoint {
    pub coord: Coordinate,
}
