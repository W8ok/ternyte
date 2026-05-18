use crate::{
    components::{base::*, ui::*},
    input,
    sdl::{
        Sdl,
        camera::Camera,
        event::{Key, MouseButton},
    },
};
use hecs::*;

pub fn new(world: &mut World) {
    world.spawn((
        Button,
        Rect {
            x: 20.,
            y: 625.,
            w: 200.,
            h: 75.,
        },
        ButtonAction::Exit,
        Color::DARKGRAY,
        Editor,
        Text("Exit".to_string()),
    ));
}

fn camera_movement(camera: &mut Camera) {
    let speed = if input::key_pressed(Key::Shift) {
        20.0
    } else {
        10.0
    };

    if input::key_pressed(Key::W) {
        camera.y -= speed / camera.zoom;
    }
    if input::key_pressed(Key::S) {
        camera.y += speed / camera.zoom;
    }
    if input::key_pressed(Key::A) {
        camera.x -= speed / camera.zoom;
    }
    if input::key_pressed(Key::D) {
        camera.x += speed / camera.zoom;
    }

    static mut LAST_POS: (f32, f32) = (0.0, 0.0);

    if input::mouse_pressed(MouseButton::Middle) {
        let pos = input::mouse_pos();
        unsafe {
            camera.x -= (pos.x - LAST_POS.0) / camera.zoom;
            camera.y -= (pos.y - LAST_POS.1) / camera.zoom;
            LAST_POS = (pos.x, pos.y);
        }
    } else {
        let pos = input::mouse_pos();
        unsafe {
            LAST_POS = (pos.x, pos.y);
        }
    }
}

pub fn interact(sdl: &mut Sdl, world: &mut World) -> bool {
    camera_movement(&mut sdl.camera);

    let result = world
        .query::<(Entity, &ButtonAction)>()
        .with::<(&Button, &Interacted, &Editor)>()
        .into_iter()
        .next()
        .map(|(entity, action)| (entity, *action));

    if let Some((entity, action)) = result {
        world.remove_one::<Interacted>(entity).unwrap();
        match action {
            ButtonAction::Exit => return false,
            _ => {}
        }
    }

    return true;
}

pub fn hover(world: &mut World) {
    let pos = input::mouse_pos();

    for (color, button) in world
        .query_mut::<(&mut Color, &Rect)>()
        .with::<(&Button, &Editor)>()
        .into_iter()
    {
        if button.contains(pos.x, pos.y) {
            *color = Color::GRAY;
        } else {
            *color = Color::DARKGRAY;
        }
    }
}

pub fn click(world: &mut World) {
    static mut CLICKED: bool = false;
    if input::mouse_pressed(MouseButton::Left) {
        let pos = input::mouse_pos();
        unsafe { CLICKED = true };

        for (color, button) in world
            .query_mut::<(&mut Color, &Rect)>()
            .with::<(&Button, &Editor)>()
            .into_iter()
        {
            if button.contains(pos.x, pos.y) {
                *color = Color::LIGHTGRAY;
            }
        }
    } else if unsafe { CLICKED } {
        let pos = input::mouse_pos();
        unsafe { CLICKED = false };

        let mut to_interact = Vec::new();
        for (entity, button) in world
            .query::<(Entity, &Rect)>()
            .with::<(&Button, &Editor)>()
            .into_iter()
        {
            if button.contains(pos.x, pos.y) {
                to_interact.push(entity);
            }
        }

        for entity in to_interact {
            world.insert(entity, (Interacted,)).unwrap();
        }
    }
}

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

pub fn render(sdl: &mut Sdl, world: &World) {
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

    sdl.text.color(Color::WHITE);

    for (rect, color, text) in world
        .query::<(&Rect, &Color, &Text)>()
        .with::<(&Button, &Editor)>()
        .iter()
    {
        sdl.render.color(color);
        sdl.render.rect(rect);
        sdl.render.color(&Color::BLACK);
        sdl.render.rect_line(rect);

        const PADDING: f32 = 10.0;
        sdl.text.size(rect.h - PADDING * 4.);
        sdl.text
            .render(&text.0, rect.x + PADDING, rect.y + PADDING * 2.);
    }
}
