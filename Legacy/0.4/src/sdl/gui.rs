use super::Render;
use crate::components::*;

pub struct GUI {}

impl GUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn grid(&mut self, render: &mut Render) {
        // Remove cam parameter, use render.camera
        const GRID_SIZE: f32 = 16.0;

        let width = render.window.w as f32;
        let height = render.window.h as f32;
        let cam = &render.camera; // Use the camera from render

        let left = cam.pos.x - (width / 2.0) * cam.zoom;
        let right = cam.pos.x + (width / 2.0) * cam.zoom;
        let top = cam.pos.y - (height / 2.0) * cam.zoom;
        let bottom = cam.pos.y + (height / 2.0) * cam.zoom;

        // Vertical lines - pass world coordinates
        let start_x = (left / GRID_SIZE).floor() * GRID_SIZE;
        let mut x = start_x;
        while x <= right {
            let start = Position::new(x, top); // World coordinates
            let end = Position::new(x, bottom); // World coordinates

            render.line(&start, &end); // render.line applies camera transform
            x += GRID_SIZE;
        }

        // Horizontal lines - pass world coordinates
        let start_y = (top / GRID_SIZE).floor() * GRID_SIZE;
        let mut y = start_y;
        while y <= bottom {
            let start = Position::new(left, y); // World coordinates
            let end = Position::new(right, y); // World coordinates

            render.line(&start, &end);
            y += GRID_SIZE;
        }
    }
}
