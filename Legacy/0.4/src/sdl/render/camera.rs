use crate::{components::*, input::Input};
use sdl3::keyboard::Keycode;

pub struct Camera {
    pub pos: Position,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Position::zero(),
            zoom: 1.0,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let move_speed = 5.0 * self.zoom;
        if input.is_key_pressed(Keycode::W) {
            self.pos.y -= move_speed;
        }
        if input.is_key_pressed(Keycode::S) {
            self.pos.y += move_speed;
        }
        if input.is_key_pressed(Keycode::A) {
            self.pos.x -= move_speed;
        }
        if input.is_key_pressed(Keycode::D) {
            self.pos.x += move_speed;
        }

        const ZOOM_SPEED: f32 = 0.05;
        if input.is_key_pressed(Keycode::Q) {
            self.zoom += ZOOM_SPEED;
            self.zoom = self.zoom.min(3.0);
        }
        if input.is_key_pressed(Keycode::E) {
            self.zoom -= ZOOM_SPEED;
            self.zoom = self.zoom.max(0.2);
        }
        if input.is_key_pressed(Keycode::R) {
            self.zoom = 1.0;
        }
    }
}

pub trait TransformToScreen {
    fn transform_to_screen(&self, cam: &Camera, window: &Size<u32>) -> Self;
}

impl TransformToScreen for Position {
    fn transform_to_screen(&self, cam: &Camera, window: &Size<u32>) -> Self {
        Position::new(
            (self.x - cam.pos.x) / cam.zoom + window.w as f32 / 2.0,
            (self.y - cam.pos.y) / cam.zoom + window.h as f32 / 2.0,
        )
    }
}

impl TransformToScreen for Rect {
    fn transform_to_screen(&self, cam: &Camera, window: &Size<u32>) -> Self {
        let pos = Position::new(self.x, self.y).transform_to_screen(cam, window);
        Rect::new(pos.x, pos.y, self.w / cam.zoom, self.h / cam.zoom)
    }
}

pub trait TransformToWorld {
    fn transform_to_world(&self, cam: &Camera, window: &Size<u32>) -> Self;
}

impl TransformToWorld for Position {
    fn transform_to_world(&self, cam: &Camera, window: &Size<u32>) -> Self {
        Position::new(
            (self.x - window.w as f32 / 2.0) * cam.zoom + cam.pos.x,
            (self.y - window.h as f32 / 2.0) * cam.zoom + cam.pos.y,
        )
    }
}

impl TransformToWorld for Rect {
    fn transform_to_world(&self, cam: &Camera, window: &Size<u32>) -> Self {
        let pos = Position::new(self.x, self.y).transform_to_world(cam, window);
        Rect::new(pos.x, pos.y, self.w * cam.zoom, self.h * cam.zoom)
    }
}
