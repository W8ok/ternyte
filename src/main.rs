#![allow(dead_code, unused)]

mod scene;
use scene::*;
mod sdl;
use sdl::{
    Sdl,
    event::{Key, MouseButton},
    types::*,
};
mod ui;
use ui::{button::ButtonAction, *};
mod event;
mod input;

fn main() {
    let mut sdl = Sdl::new("goob", 1280, 720);
    let mut ui = Ui::new();
    let mut scene = Scene::new(SceneSelect::Editor);

    sdl.text.load("assets/font.ttf");
    ui.add_button(
        Rect {
            x: 20.,
            y: 625.,
            w: 200.,
            h: 75.,
        },
        "Exit",
        ButtonAction::Exit,
    );

    'main: loop {
        if !event::handle(&mut sdl) {
            break 'main;
        }

        if !scene.interract(&mut sdl, &mut ui) {
            break 'main;
        }

        sdl.render.clear(Color::BLACK);

        scene.render(&mut sdl, &mut ui);

        sdl.render.present();
    }

    sdl.exit();
}
