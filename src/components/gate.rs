use super::base::*;

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
pub struct InputPoints(pub Vec<Position>);
