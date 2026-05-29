#![allow(dead_code, unused)]
#![windows_subsystem = "windows"]
use hecs::*;

mod components;
use components::{base::*, gate::*, ui::*};
mod scene;
use scene::*;
mod sdl;
use sdl::{Sdl, types::*};
mod debug;
mod event;
mod input;

const FONT_BYTES: &[u8] = include_bytes!("../assets/font.ttf");

fn main() {
    let mut sdl = Sdl::new("Ternyte v0.0.1", 1280, 720);
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
