use super::GRID_SIZE;
use super::helpers::*;
use crate::{
    components::{base::*, gate::*, ui::*},
    input,
    sdl::{
        Sdl,
        camera::Camera,
        event::{Key, MouseButton},
        types::*,
    },
};
use hecs::*;

enum WireState {
    None,    // Disconnected
    False,   // Binary 0,   Ternary -1
    Neutral, // Binary 0,   Ternary 0
    True,    // Binary 1,   Ternary 1
}

struct Wire {
    state: WireState,
    start: Position,
    end: Position,
    start_placed: bool,
    end_placed: bool,
    moving: bool,
}

pub fn add_wire(world: &mut World) {
    let mut tool = world
        .query_mut::<&mut Tool>()
        .with::<&Resource>()
        .without::<&Placed>()
        .into_iter()
        .next()
        .unwrap();
    if *tool != Tool::Wire || !input::mouse_pressed(MouseButton::Left) {
        return;
    }

    world.spawn((Wire {
        state: WireState::True,
        start: Position::default(),
        end: Position::default(),
        start_placed: false,
        end_placed: false,
        moving: false,
    },));
}

pub fn place_wire(world: &mut World) {
    let to_place: Vec<Entity> = world
        .query::<Entity>()
        .with::<&Wire>()
        .without::<&Placed>()
        .into_iter()
        .collect();

    let mut tool = world
        .query_mut::<&mut Tool>()
        .with::<&Resource>()
        .without::<&Placed>()
        .into_iter()
        .next()
        .unwrap();
    if *tool != Tool::Wire {
        for entity in to_place {
            world.despawn(entity);
        }
        return;
    }

    let pos = snap_to_grid(input::mouse_pos_camera());

    for entity in to_place {
        let mut wire = world.get::<&mut Wire>(entity).unwrap();

        if !wire.start_placed {
            wire.start = pos;
            if !wire.end_placed {
                wire.end = pos;
            }
            if input::mouse_pressed(MouseButton::Left) {
                wire.start_placed = true;
                return;
            }
        }

        if !wire.end_placed {
            wire.end = pos;
            if !wire.start_placed {
                wire.start = pos;
            }
            if input::mouse_pressed(MouseButton::Left) {
                wire.end_placed = true;
                drop(wire);
                world.insert_one(entity, Placed).unwrap();
                return;
            }
        }

        if wire.moving {
            return;
        }

        drop(wire);
        if input::key_pressed(Key::Escape) || input::mouse_pressed(MouseButton::Right) {
            world.despawn(entity);
        }
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    for (entity, wire) in world.query::<(Entity, &Wire)>().iter() {
        let color = match wire.state {
            WireState::None => Color::BLACK,
            WireState::False => Color::RED,
            WireState::Neutral => Color::BLUE,
            WireState::True => Color::GREEN,
        };

        sdl.render.color(&color);
        sdl.render
            .line(wire.start.x, wire.start.y, wire.end.x, wire.end.y);

        if world.get::<&Selected>(entity).is_ok() {
            sdl.render.color(&Color::LIGHTBLUE);
            sdl.render
                .line(wire.start.x, wire.start.y, wire.end.x, wire.end.y);
        }
    }
}
