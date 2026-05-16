#![allow(dead_code, unused)]
use sdl3_sys::everything::*;
use std::{ffi::CString, ptr};

pub mod types;
use types::*;
pub mod render;
use render::Render;
pub mod event;
use event::SdlEvent;
pub mod text;
use text::Text;
pub mod texture;
use texture::Texture;
pub mod camera;
use camera::Camera;

pub struct Sdl {
    pub render: Render,
    pub text: Text,
    pub texture: Texture,
    pub event: SdlEvent,
    pub camera: Box<Camera>,

    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer,
}

impl Sdl {
    pub fn new(name: &str, size_x: i32, size_y: i32) -> Self {
        let mut window = ptr::null_mut();
        let mut renderer = ptr::null_mut();

        let name = CString::new(name).unwrap();

        unsafe {
            SDL_Init(SDL_INIT_VIDEO);
            SDL_CreateWindowAndRenderer(
                name.as_ptr(),
                size_x,
                size_y,
                SDL_WindowFlags(0),
                &mut window,
                &mut renderer,
            );
        }

        let mut camera = Box::new(Camera::new(0., 0., 1.));
        let camera_ptr = &mut *camera as *mut Camera;

        let render = Render::new(renderer, camera_ptr);
        let text = Text::new(renderer, camera_ptr);
        let texture = Texture::new(renderer, camera_ptr);
        let event = SdlEvent::new();

        Self {
            window,
            renderer,
            render,
            text,
            texture,
            event,
            camera,
        }
    }

    pub fn exit(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }

    pub fn delay(&self, ms: u32) {
        unsafe {
            SDL_Delay(ms);
        }
    }

    pub fn get_window_size(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            SDL_GetWindowSize(self.window, &mut w, &mut h);
        }

        return (w, h);
    }
}
