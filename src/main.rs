#![allow(dead_code, unused)]
#![windows_subsystem = "windows"]
use hecs::*;

mod components;
use components::{base::*, ui::*};
mod scene;
use scene::*;
mod sdl;
use sdl::{
    Sdl,
    event::{Key, MouseButton},
    types::*,
};
mod event;
mod input;

mod debug {
    use std::cell::RefCell;
    use std::time::Instant;

    thread_local! {
        static LAST_PRINT: RefCell<Instant> = RefCell::new(Instant::now());
        static FRAME_COUNT: RefCell<u32> = const { RefCell::new(0) };
    }

    pub fn tick() {
        FRAME_COUNT.with(|count| {
            let mut count = count.borrow_mut();
            *count += 1;

            LAST_PRINT.with(|last| {
                if last.borrow().elapsed().as_secs_f32() >= 1.0 {
                    println!(
                        "FPS: {} | Frame time: {:.2}ms",
                        *count,
                        1000.0 / *count as f32
                    );
                    *count = 0;
                    *last.borrow_mut() = Instant::now();
                }
            });
        });
    }
}

fn main() {
    let mut sdl = Sdl::new("Ternyte v0.0.1", 1280, 720);

    const FONT_BYTES: &[u8] = include_bytes!("../assets/font.ttf");
    sdl.text.load_from_bytes(FONT_BYTES);

    let mut world = World::new();
    world.spawn((SceneSelect::Editor, Resource));
    scene::builder(&mut sdl, &mut world);

    'main: loop {
        if !event::handle(&mut sdl, &mut world) {
            break 'main;
        }

        sdl.render.clear(Color::GRAY);

        if !scene::manager(&mut sdl, &mut world) {
            break 'main;
        }

        sdl.render.present();
        debug::tick();
    }

    sdl.exit();
}
