use crate::{
    components::{base::*, ui::*},
    input,
    sdl::{Sdl, event::MouseButton},
};
use hecs::*;

pub fn new(sdl: &mut Sdl, world: &mut World) {
    let (width, height) = sdl.get_window_size();
    world.spawn((
        Button,
        Rect {
            x: 20.,
            y: height as f32 - 95.,
            w: 200.,
            h: 75.,
        },
        ButtonAction::Exit,
        Color::DARKGRAY,
        Text("Exit"),
        Menu,
        Ui,
    ));
}

pub fn interact(world: &mut World) -> bool {
    let result = world
        .query::<(Entity, &ButtonAction)>()
        .with::<(&Button, &Interacted, &Menu)>()
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
        .with::<(&Button, &Menu)>()
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
            .with::<(&Button, &Menu)>()
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
            .with::<(&Button, &Menu)>()
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

pub fn render(sdl: &mut Sdl, world: &World) {
    let (w, h) = sdl.get_window_size();

    sdl.text.color(Color::WHITE);

    for (rect, color, text) in world
        .query::<(&Rect, &Color, &Text)>()
        .with::<(&Button, &Menu)>()
        .iter()
    {
        sdl.render.color(color);
        sdl.render.rect(rect);
        sdl.render.color(&Color::BLACK);
        sdl.render.rect_line(rect);

        const PADDING: f32 = 10.0;
        sdl.text.size(rect.h - PADDING * 4.);
        sdl.text
            .render(text.0, rect.x + PADDING, rect.y + PADDING * 2.);
    }
}
