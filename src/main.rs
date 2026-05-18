#![allow(dead_code, unused)]
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

fn main() {
    let mut sdl = Sdl::new("goob", 1280, 720);
    let mut world = World::new();
    world.spawn((SceneSelect::Editor, Resource));

    sdl.text.load("assets/font.ttf");
    scene::editor::new(&mut world);

    'main: loop {
        if !event::handle(&mut sdl) {
            break 'main;
        }

        sdl.render.clear(Color::BLACK);

        if !scene::manager(&mut sdl, &mut world) {
            break 'main;
        }

        sdl.render.present();
    }

    sdl.exit();
}
