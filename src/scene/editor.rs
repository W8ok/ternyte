use crate::input;
use crate::sdl::{Sdl, camera::Camera, event::*, types::*};
use crate::ui::{button::ButtonAction, *};

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

pub fn render(sdl: &mut Sdl) {
    sdl.camera.start();
    {
        grid(sdl);
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
}

fn input(camera: &mut Camera) {
    let camera_speed = if input::key_pressed(Key::Shift) {
        20.0
    } else {
        10.0
    };

    if input::key_pressed(Key::W) {
        camera.y -= camera_speed / camera.zoom;
    }
    if input::key_pressed(Key::A) {
        camera.x -= camera_speed / camera.zoom;
    }
    if input::key_pressed(Key::S) {
        camera.y += camera_speed / camera.zoom;
    }
    if input::key_pressed(Key::D) {
        camera.x += camera_speed / camera.zoom;
    }
}

pub fn interract(sdl: &mut Sdl, ui: &mut Ui) -> bool {
    input(&mut sdl.camera);
    ui.update();

    for button in ui.buttons.iter() {
        if button.pressed {
            match button.action {
                ButtonAction::Exit => return false,
            }
        }
    }

    return true;
}
