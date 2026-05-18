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
    sdl.text.load("assets/font.ttf");

    let mut world = World::new();
    world.spawn((SceneSelect::Editor, Resource));
    scene::builder(&mut sdl, &mut world);

    'main: loop {
        if !event::handle(&mut sdl, &mut world) {
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
