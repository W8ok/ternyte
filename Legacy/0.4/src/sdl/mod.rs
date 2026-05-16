#[path = "render/mod.rs"]
pub mod render;
use render::Render;

use crate::components::*;

pub mod resource;
use resource::Resource;

pub mod gui;
use gui::GUI;

pub mod event;
use event::SDLEvent;

use std::{cell::RefCell, rc::Rc};

pub struct SDL {
    pub event: SDLEvent,
    pub render: Render,
    pub gui: GUI,
    pub resource: Resource,
}

impl SDL {
    pub fn init() -> Self {
        // Unsafe because its a compile env command...
        // Couldn't find any better way...
        unsafe {
            std::env::set_var("SDL_RENDER_VSYNC", "-1");
        }

        let sdl = sdl3::init().unwrap();
        let video = sdl.video().unwrap();

        let window_size = Size::new(1280, 720);
        let window = video
            .window("Ternyte", window_size.w, window_size.h)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();

        let event = SDLEvent::new(&sdl);
        let resource = Resource::new(&canvas);
        let render = Render::new(&sdl, canvas, window_size);
        let gui = GUI::new();

        Self {
            event,
            render,
            gui,
            resource,
        }
    }
}
