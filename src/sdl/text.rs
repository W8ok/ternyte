use sdl3_sys::everything::*;
use sdl3_ttf_sys::everything::*;
use std::{ffi::CString, ptr};

use super::{Camera, types::*};

pub struct Text {
    renderer: *mut SDL_Renderer,
    camera: *mut Camera,
    font: *mut TTF_Font,
    color: SDL_Color,
}

impl Text {
    pub fn new(renderer: *mut SDL_Renderer, camera: *mut Camera) -> Self {
        unsafe { TTF_Init() };
        Self {
            renderer,
            camera,
            font: ptr::null_mut(),
            color: SDL_Color::default(),
        }
    }

    pub fn load(&mut self, path: &str) {
        unsafe {
            let path = CString::new(path).unwrap();
            self.font = TTF_OpenFont(path.as_ptr(), 24.0);

            if self.font.is_null() {
                println!("Failed to load font!")
            }
        }
    }

    pub fn size(&mut self, size: f32) {
        unsafe { TTF_SetFontSize(self.font, size) };
    }

    pub fn color(&mut self, color: Color) {
        self.color = SDL_Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
    }

    // Future me... just dont... dont touch it...
    // Could optimize by making it generate the texture once... but no :)
    pub fn render(&mut self, text: &str, x: f32, y: f32) {
        unsafe {
            let text = CString::new(text).unwrap();
            let surface = TTF_RenderText_Blended(self.font, text.as_ptr(), 0, self.color);
            assert!(!surface.is_null());

            let w = (*surface).w;
            let h = (*surface).h;

            let texture = SDL_CreateTextureFromSurface(self.renderer, surface);
            assert!(!texture.is_null());

            let camera = unsafe { &*self.camera };
            let rect = if camera.active {
                SDL_FRect {
                    x: (x - camera.x) * camera.zoom,
                    y: (y - camera.y) * camera.zoom,
                    w: w as f32 * camera.zoom,
                    h: h as f32 * camera.zoom,
                }
            } else {
                SDL_FRect {
                    x,
                    y,
                    w: w as f32,
                    h: h as f32,
                }
            };

            SDL_RenderTexture(self.renderer, texture, ptr::null(), &rect);

            SDL_DestroyTexture(texture);
            SDL_DestroySurface(surface);
        }
    }
}
