use glam::Vec2;
use raylib_sys::*;
use std::ffi::CString;

use crate::components::*;

pub use raylib_sys::Color;

#[derive(Default)]
pub struct Camera {
    pub target: Vec2,
    pub offset: Vec2,
    pub rotation: f32,
    pub zoom: f32,
}

impl Camera {
    fn to_rl(&self) -> Camera2D {
        Camera2D {
            offset: Vector2 {
                x: self.offset.x,
                y: self.offset.y,
            },
            target: Vector2 {
                x: self.target.x,
                y: self.target.y,
            },
            rotation: self.rotation,
            zoom: self.zoom,
        }
    }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() };
}

pub fn end_drawing() {
    unsafe { EndDrawing() };
}

pub fn begin_mode2d(camera: &Camera) {
    unsafe { BeginMode2D(camera.to_rl()) };
}

pub fn end_mode2d() {
    unsafe { EndMode2D() };
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) };
}

pub fn draw_fps(x: i32, y: i32) {
    unsafe { DrawFPS(x, y) };
}

pub fn draw_line(start: &Vec2, end: &Vec2, color: Color) {
    let start = Vector2 {
        x: start.x,
        y: start.y,
    };
    let end = Vector2 { x: end.x, y: end.y };

    unsafe { DrawLineV(start, end, color) };
}

pub fn draw_rect(rect: &Rect, color: Color) {
    unsafe { DrawRectangle(rect.x, rect.y, rect.w, rect.h, color) };
}

pub fn draw_rect_lines(rect: &Rect, color: Color) {
    unsafe { DrawRectangleLines(rect.x, rect.y, rect.w, rect.h, color) };
}

pub fn draw_circle(coord: &Coordinate, radius: f32, color: Color) {
    unsafe { DrawCircle(coord.x, coord.y, radius, color) };
}

pub fn draw_text(text: &str, coord: Coordinate, size: i32, color: Color) {
    unsafe {
        DrawText(
            CString::new(text).unwrap().as_ptr(),
            coord.x,
            coord.y,
            size,
            color,
        )
    };
}

pub fn measure_text(text: &str, font_size: i32) -> i32 {
    unsafe { MeasureText(CString::new(text).unwrap().as_ptr(), font_size) }
}

pub fn center_text(text_width: i32, font_size: i32, rect: &Rect) -> Coordinate {
    Coordinate::new(
        rect.x + (rect.w - text_width) / 2,
        rect.y + (rect.h + font_size) / 2 - font_size,
    )
}

pub fn open_url(url: &str) {
    unsafe { OpenURL(CString::new(url).unwrap().as_ptr()) };
}
