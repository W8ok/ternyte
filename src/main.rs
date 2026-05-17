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

fn grid(sdl: &mut Sdl) {
    sdl.render.color(&Color::GREEN);

    let (w, h) = sdl.get_window_size();
    let camera = &sdl.camera;

    let spacing = 16.0;

    let left = camera.x;
    let top = camera.y;
    let right = left + w as f32 / camera.zoom;
    let bottom = top + h as f32 / camera.zoom;

    // I have no real idea wtf this does... :3
    let snap = |v: f32| (v / spacing).floor() * spacing;

    let mut x = snap(left);
    while x <= right {
        sdl.render.line(x, top, x, bottom);
        x += spacing;
    }

    let mut y = snap(top);
    while y <= bottom {
        sdl.render.line(left, y, right, y);
        y += spacing;
    }
}

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

        if input::key_pressed(Key::W) {
            sdl.camera.y -= 10.0;
        }
        if input::key_pressed(Key::A) {
            sdl.camera.x -= 10.0;
        }
        if input::key_pressed(Key::S) {
            sdl.camera.y += 10.0;
        }
        if input::key_pressed(Key::D) {
            sdl.camera.x += 10.0;
        }

        sdl.camera
            .update(sdl.camera.x, sdl.camera.y, sdl.camera.zoom);

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
        {
            grid(&mut sdl);
        }
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
