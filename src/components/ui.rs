pub use super::gate;
pub use crate::sdl::types::*;

pub struct Ui;
pub struct Editor;
pub struct Menu;

pub struct Button;
pub struct Interacted;

#[derive(Clone, Copy)]
pub enum ButtonAction {
    Exit,
    Wire,
    Select,
    AddGateAND,
    AddGateOR,
    AddGateXOR,
    AddGateNOT,
    AddGateNAND,
    AddGateNOR,
    AddGateXNOR,
    AddGateBUF,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Tool {
    None,
    Wire,
    Select,
}

#[derive(Clone, Copy)]
pub struct SelectRect {
    pub rect: Rect,
}

#[derive(Clone, Copy)]
pub struct DeselectRect {
    pub rect: Rect,
}
