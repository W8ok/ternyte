// src/input.rs
use crate::sdl::event::{Key, MouseButton};
use std::cell::RefCell;

thread_local! {
    pub static KEYS: RefCell<Vec<Key>> = const { RefCell::new(Vec::new()) };
    pub static MOUSE_BUTTONS: RefCell<Vec<MouseButton>> = const { RefCell::new(Vec::new()) };
    pub static MOUSE_X: RefCell<f32> = const { RefCell::new(0.0) };
    pub static MOUSE_Y: RefCell<f32> = const { RefCell::new(0.0) };
}

pub fn key_pressed(key: Key) -> bool {
    KEYS.with(|k| k.borrow().contains(&key))
}

pub fn mouse_pressed(button: MouseButton) -> bool {
    MOUSE_BUTTONS.with(|b| b.borrow().contains(&button))
}

pub fn mouse_pos() -> (f32, f32) {
    (MOUSE_X.with(|x| *x.borrow()), MOUSE_Y.with(|y| *y.borrow()))
}
