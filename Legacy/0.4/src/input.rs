#![allow(dead_code)]

use crate::{components::*, sdl::render::Render};
use sdl3::{keyboard::Keycode, mouse::MouseButton};
use std::collections::HashSet;

pub struct Input {
    keys: HashSet<Keycode>,
    mouse: HashSet<MouseButton>,
    render: *const Render,
    mouse_pos: Position,
}

impl Input {
    pub fn new(render: &Render) -> Self {
        Self {
            keys: HashSet::new(),
            mouse: HashSet::new(),
            mouse_pos: Position::zero(),
            render,
        }
    }

    #[inline]
    pub fn press_key(&mut self, key: Keycode) {
        self.keys.insert(key);
    }

    #[inline]
    pub fn release_key(&mut self, key: Keycode) {
        self.keys.remove(&key);
    }

    #[inline]
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        return self.keys.contains(&key);
    }

    #[inline]
    pub fn click_mouse(&mut self, mouse_btn: MouseButton) {
        self.mouse.insert(mouse_btn);
    }

    #[inline]
    pub fn release_mouse(&mut self, mouse_btn: MouseButton) {
        self.mouse.remove(&mouse_btn);
    }

    #[inline]
    pub fn is_mouse_clicked(&self, mouse_btn: MouseButton) -> bool {
        return self.mouse.contains(&mouse_btn);
    }

    #[inline]
    pub fn set_mouse_pos(&mut self, pos: Position) {
        self.mouse_pos = pos;
    }

    #[inline]
    pub fn get_mouse_pos(&self) -> Position {
        // Goober C code in my Rust go brrrrrr
        let render = unsafe { &*self.render };
        return render.to_world(&Position::new(self.mouse_pos.x, self.mouse_pos.y));
    }
}
