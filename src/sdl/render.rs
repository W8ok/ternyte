use sdl3_sys::everything::*;
use std::{ffi::CString, ptr};

use super::{Camera, types::*};

pub struct Render {
    renderer: *mut SDL_Renderer,
    camera: *mut Camera,
}

impl Render {
    pub fn new(renderer: *mut SDL_Renderer, camera: *mut Camera) -> Self {
        Self { renderer, camera }
    }

    pub fn present(&mut self) {
        unsafe {
            SDL_RenderPresent(self.renderer);
        }
    }

    pub fn color(&mut self, color: &Color) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, color.r, color.g, color.b, color.a);
            SDL_RenderClear(self.renderer);
        }
    }

    // ===================
    // Primitives
    // ===================
    pub fn rect(&mut self, rect: &Rect) {
        let camera = unsafe { &*self.camera };

        let rect = if camera.active {
            SDL_FRect {
                x: (rect.x - camera.x) * camera.zoom,
                y: (rect.y - camera.y) * camera.zoom,
                w: rect.w * camera.zoom,
                h: rect.h * camera.zoom,
            }
        } else {
            SDL_FRect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: rect.h,
            }
        };

        unsafe {
            SDL_RenderFillRect(self.renderer, &rect);
        }
    }

    pub fn rect_line(&mut self, rect: &Rect) {
        let camera = unsafe { &*self.camera };

        let rect = if camera.active {
            SDL_FRect {
                x: (rect.x - camera.x) * camera.zoom,
                y: (rect.y - camera.y) * camera.zoom,
                w: rect.w * camera.zoom,
                h: rect.h * camera.zoom,
            }
        } else {
            SDL_FRect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: rect.h,
            }
        };

        unsafe {
            SDL_RenderRect(self.renderer, &rect);
        }
    }
}
