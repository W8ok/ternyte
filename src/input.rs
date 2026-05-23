use crate::components::base::*;
use crate::sdl::event::{Key, MouseButton};
use std::cell::RefCell;

thread_local! {
    pub static KEYS: RefCell<Vec<Key>> = const { RefCell::new(Vec::new()) };
    pub static MOUSE_BUTTONS: RefCell<Vec<MouseButton>> = const { RefCell::new(Vec::new()) };
    pub static MOUSE_X: RefCell<f32> = const { RefCell::new(0.0) };
    pub static MOUSE_Y: RefCell<f32> = const { RefCell::new(0.0) };
    pub static MOUSE_X_CAM: RefCell<f32> = const { RefCell::new(0.0) };
    pub static MOUSE_Y_CAM: RefCell<f32> = const { RefCell::new(0.0) };
}

pub fn key_pressed(key: Key) -> bool {
    KEYS.with(|k| k.borrow().contains(&key))
}

pub fn mouse_pressed(button: MouseButton) -> bool {
    MOUSE_BUTTONS.with(|b| b.borrow().contains(&button))
}

pub fn mouse_pos() -> Position {
    let x = MOUSE_X.with(|x| *x.borrow());
    let y = MOUSE_Y.with(|y| *y.borrow());
    Position { x, y }
}

pub fn mouse_pos_camera() -> Position {
    let x = MOUSE_X_CAM.with(|x| *x.borrow());
    let y = MOUSE_Y_CAM.with(|y| *y.borrow());
    Position { x, y }
}
