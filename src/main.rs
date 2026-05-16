#![allow(dead_code, unused)]

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

        ui.update();

        for button in ui.buttons.iter() {
            if button.pressed {
                match button.action {
                    ButtonAction::Exit => break 'main,
                }
            }
        }

        sdl.render.clear(Color::BLACK);
        sdl.camera.start();
        {}
        sdl.camera.end();

        let (w, h) = sdl.get_window_size();
        let side_panel = Rect {
            x: 0.0,
            y: 0.0,
            w: 240.0,
            h: h as f32,
        };

        sdl.render.color(&Color::DARKGRAY);
        sdl.render.rect(&side_panel);
        sdl.render.color(&Color::BLACK);
        sdl.render.rect_line(&side_panel);

        ui.render(&mut sdl);
        sdl.render.present();
    }

    sdl.exit();
}
