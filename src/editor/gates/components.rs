use crate::components::*;

#[derive(Debug, Clone, Copy)]
pub enum GateKind {
    // Binary
    AND,
    NAND,
    OR,
    NOR,
    NOT,
    BUF,
    XOR,
    XNOR,
}

pub struct Gate {
    pub kind: GateKind,
    pub rect: Rect,
}
