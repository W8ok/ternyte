#![allow(dead_code)]

use crate::{components::*, input::Input};

pub mod camera;
use camera::*;

use super::resource::*;

use sdl3::{
    image,
    keyboard::Keycode,
    pixels::Color,
    render::*,
    ttf,
    ttf::{Font, Sdl3TtfContext},
    video::WindowContext,
};

use std::{cell::RefCell, rc::Rc};

pub struct Render {
    pub window: Size<u32>,
    pub camera: Camera,
    pub canvas: WindowCanvas,
    pub resource: *mut Resource,
}

impl Render {
    pub fn new(sdl: &sdl3::Sdl, canvas: WindowCanvas, window: Size<u32>) -> Self {
        Self {
            window,
            camera: Camera::new(),
            canvas,
            resource: std::ptr::null_mut(),
        }
    }

    #[inline]
    pub fn to_screen<T: TransformToScreen>(&self, thing: &T) -> T {
        thing.transform_to_screen(&self.camera, &self.window)
    }

    #[inline]
    pub fn to_world<T: TransformToWorld>(&self, thing: &T) -> T {
        thing.transform_to_world(&self.camera, &self.window)
    }

    #[inline]
    pub fn set_window_size(&mut self, size: Size<u32>) {
        self.window = size;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    #[inline]
    pub fn present(&mut self) {
        self.canvas.present();
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    #[inline]
    pub fn line(&mut self, start: &Position, end: &Position) {
        let screen_start = start.transform_to_screen(&self.camera, &self.window);
        let screen_end = end.transform_to_screen(&self.camera, &self.window);
        self.canvas
            .draw_line(
                sdl3::render::FPoint::new(screen_start.x, screen_start.y),
                sdl3::render::FPoint::new(screen_end.x, screen_end.y),
            )
            .unwrap();
    }

    #[inline]
    pub fn rectangle(&mut self, rect: &Rect) {
        let screen_rect = rect.transform_to_screen(&self.camera, &self.window);
        self.canvas
            .fill_rect(FRect::new(
                screen_rect.x,
                screen_rect.y,
                screen_rect.w,
                screen_rect.h,
            ))
            .unwrap();
    }

    #[inline]
    pub fn texture(&mut self, texture: &Texture, src: Option<&Rect>, dst: &Rect) {
        let screen_dst = dst.transform_to_screen(&self.camera, &self.window);
        self.canvas
            .copy(
                texture,
                src.map(|src| FRect::new(src.x, src.y, src.w, src.h)),
                Some(FRect::new(
                    screen_dst.x,
                    screen_dst.y,
                    screen_dst.w,
                    screen_dst.h,
                )),
            )
            .unwrap();
    }

    #[inline]
    pub fn text(&mut self, text: &str, pos: Position, size: f32, color: Color) {
        let mut resource = unsafe { &mut *self.resource };
        let font = resource.font.as_mut().unwrap();
        font.set_size(size).unwrap();

        let surface = font.render(text).blended(color).unwrap();
        let texture = resource
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let dst = FRect::new(
            pos.x,
            pos.y,
            surface.width() as f32,
            surface.height() as f32,
        );

        self.canvas.copy(&texture, None, dst).unwrap();
    }
}
