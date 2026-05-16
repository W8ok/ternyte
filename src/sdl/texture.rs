use sdl3_sys::everything::*;
use std::{ffi::CString, ptr};

use super::{Camera, types::*};

pub struct Texture {
    renderer: *mut SDL_Renderer,
    camera: *mut Camera,
    textures: Vec<*mut SDL_Texture>,
}

impl Texture {
    pub fn new(renderer: *mut SDL_Renderer, camera: *mut Camera) -> Self {
        Self {
            renderer,
            camera,
            textures: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str, id: usize) {
        unsafe {
            let path = CString::new(path).unwrap();
            let surface = SDL_LoadPNG(path.as_ptr());
            let texture = SDL_CreateTextureFromSurface(self.renderer, surface);
            SDL_DestroySurface(surface);

            if id >= self.textures.len() {
                self.textures.resize(id + 1, ptr::null_mut());
            }

            self.textures[id] = texture;
        }
    }

    // My beloved :3
    pub fn render<'a>(&mut self, id: usize, src: impl Into<Option<&'a Rect>>, dst: &Rect) {
        if let Some(&texture) = self.textures.get(id) {
            if texture.is_null() {
                return;
            }

            unsafe {
                // haha shadowing go brrrrrrrrrrrrrr
                let src = match src.into() {
                    Some(src) => {
                        let src = SDL_FRect {
                            x: src.x,
                            y: src.y,
                            w: src.w,
                            h: src.h,
                        };
                        &src as *const SDL_FRect
                    }
                    None => ptr::null(),
                };

                let camera = unsafe { &*self.camera };
                let dst = if camera.active {
                    SDL_FRect {
                        x: (dst.x - camera.x) * camera.zoom,
                        y: (dst.y - camera.y) * camera.zoom,
                        w: dst.w * camera.zoom,
                        h: dst.h * camera.zoom,
                    }
                } else {
                    SDL_FRect {
                        x: dst.x,
                        y: dst.y,
                        w: dst.w,
                        h: dst.h,
                    }
                };

                SDL_RenderTexture(self.renderer, texture, src, &dst);
            }
        }
    }

    pub fn destroy(&mut self, id: usize) {
        if let Some(&texture) = self.textures.get(id) {
            if !texture.is_null() {
                unsafe {
                    SDL_DestroyTexture(texture);
                }
            }

            if id < self.textures.len() {
                self.textures[id] = ptr::null_mut();
            }
        }
    }
}
