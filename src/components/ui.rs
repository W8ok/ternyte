pub use crate::sdl::types::*;

pub struct Ui;
pub struct Editor;
pub struct Menu;

pub struct Button;
pub struct Interacted;

#[derive(Clone, Copy)]
pub enum ButtonAction {
    Exit,
}
