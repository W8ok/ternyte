use super::GRID_SIZE;
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

#[inline]
fn snap_to_grid(pos: Position) -> Position {
    Position {
        x: (pos.x / GRID_SIZE).floor() * GRID_SIZE,
        y: (pos.y / GRID_SIZE).floor() * GRID_SIZE,
    }
}

pub fn add_gate(gate_type: GateType, input_count: u8, text: Text, world: &mut World) {
    let pos = snap_to_grid(input::mouse_pos_camera());
    let rect = Rect {
        x: pos.x,
        y: pos.y,
        w: 64.,
        h: 16. + 16. * input_count as f32,
    };
    let input_points: Vec<Position> = (0..input_count)
        .map(|i| Position {
            x: rect.x,
            y: rect.y + 16. + 16. * i as f32,
        })
        .collect();

    world.spawn((rect, gate_type, InputPoints(input_points), text));

    // Set tool to place and deselect all entities
    let tool = world
        .query::<Entity>()
        .with::<(&Tool, &Resource)>()
        .iter()
        .next()
        .unwrap();
    *world.get::<&mut Tool>(tool).unwrap() = Tool::Place;

    let selected: Vec<Entity> = world.query::<Entity>().with::<&Selected>().iter().collect();

    for entity in selected {
        world.remove_one::<Selected>(entity).unwrap();
    }
}

pub fn place_gate(world: &mut World) {
    let pos = snap_to_grid(input::mouse_pos_camera());

    let to_place: Vec<Entity> = world
        .query::<Entity>()
        .with::<&GateType>()
        .without::<&Placed>()
        .into_iter()
        .collect();

    for entity in to_place {
        let mut rect = world.get::<&mut Rect>(entity).unwrap();
        rect.x = pos.x;
        rect.y = pos.y;
        drop(rect);

        let count = world.get::<&InputPoints>(entity).unwrap().0.len();
        let mut points = Vec::new();
        for i in 0..count {
            points.push(Position {
                x: pos.x,
                y: pos.y + GRID_SIZE + GRID_SIZE * i as f32,
            });
        }
        *world.get::<&mut InputPoints>(entity).unwrap() = InputPoints(points);

        if input::mouse_pressed(MouseButton::Left) {
            world.insert_one(entity, Placed).unwrap();
        } else if input::key_pressed(Key::Escape) || input::mouse_pressed(MouseButton::Right) {
            world.despawn(entity);
        }
    }
}

fn move_gates(world: &mut World) {
    let mut to_move = Vec::new();
    for entity in world
        .query::<Entity>()
        .with::<(&Selected, &Placed, &Rect, &InputPoints)>()
        .without::<&Moving>()
        .iter()
    {
        to_move.push(entity);
    }

    let mouse_pos = input::mouse_pos_camera();
    for entity in to_move {
        let rect = *world.get::<&Rect>(entity).unwrap();
        let points = world.get::<&InputPoints>(entity).unwrap().0.clone();
        let _ = rect; // Makes the borrow checker stfu

        // Rustfmt wtf (╥‸╥)
        world
            .insert_one(
                entity,
                Moving {
                    original_rect: rect,
                    original_points: points,
                    offset: Position {
                        x: rect.x - mouse_pos.x,
                        y: rect.y - mouse_pos.y,
                    },
                },
            )
            .unwrap();
    }
}

fn update_move(world: &mut World) {
    let mouse_pos = input::mouse_pos_camera();

    for (entity, moving) in world.query::<(Entity, &Moving)>().iter() {
        let new_x = mouse_pos.x + moving.offset.x;
        let new_y = mouse_pos.y + moving.offset.y;

        let snapped = snap_to_grid(Position { x: new_x, y: new_y });

        let mut rect = world.get::<&mut Rect>(entity).unwrap();
        rect.x = snapped.x;
        rect.y = snapped.y;

        let input_count = moving.original_points.len();
        let mut new_points = Vec::new();
        for i in 0..input_count {
            new_points.push(Position {
                x: snapped.x,
                y: snapped.y + GRID_SIZE + GRID_SIZE * i as f32,
            });
        }
        *world.get::<&mut InputPoints>(entity).unwrap() = InputPoints(new_points);
    }

    // Commit move
    if input::mouse_pressed(MouseButton::Left) {
        let entities: Vec<Entity> = world.query::<Entity>().with::<&Moving>().iter().collect();

        for entity in entities {
            world.remove_one::<Moving>(entity).unwrap();
            world.insert_one(entity, Placed).unwrap();
        }
    }

    // Cancel move
    if input::key_pressed(Key::Escape) || input::mouse_pressed(MouseButton::Right) {
        let entities: Vec<(Entity, Moving)> = world
            .query::<(Entity, &Moving)>()
            .iter()
            .map(|(entity, moving)| (entity, moving.clone()))
            .collect();

        for (entity, moving) in entities {
            *world.get::<&mut Rect>(entity).unwrap() = moving.original_rect;
            *world.get::<&mut InputPoints>(entity).unwrap() = InputPoints(moving.original_points);
            world.remove_one::<Moving>(entity).unwrap();
            world.insert_one(entity, Placed).unwrap();
        }
    }
}

pub fn select_gate(world: &mut World) {
    if input::key_pressed(Key::Escape) || input::mouse_pressed(MouseButton::Right) {
        let selected: Vec<Entity> = world.query::<Entity>().with::<&Selected>().iter().collect();

        for entity in selected {
            world.remove_one::<Selected>(entity).unwrap();
        }
    } else if (input::key_pressed(Key::X) && input::key_pressed(Key::Control))
        || input::key_pressed(Key::Delete)
    {
        let selected: Vec<Entity> = world.query::<Entity>().with::<&Selected>().iter().collect();

        for entity in selected {
            world.despawn(entity);
        }
    } else if input::key_pressed(Key::M) {
        move_gates(world);
    }

    update_move(world);

    if !input::mouse_pressed(MouseButton::Left) {
        return;
    }

    let pos = input::mouse_pos_camera();

    let mut to_select = Vec::new();
    for (entity, rect) in world
        .query::<(Entity, &Rect)>()
        .with::<(&Placed, &GateType)>()
        .without::<(&Selected)>()
        .iter()
    {
        if rect.contains(pos.x, pos.y) {
            to_select.push(entity);
        }
    }

    for entity in to_select {
        world.insert_one(entity, Selected).unwrap();
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    for (entity, rect, gate_type, input_points, text) in world
        .query::<(Entity, &Rect, &GateType, &InputPoints, &Text)>()
        .iter()
    {
        let color = match gate_type {
            GateType::AND => Color::RED,
            GateType::OR => Color::BLUE,
            GateType::XOR => Color::MAGENTA,
            GateType::NOT => Color::LIME,
            GateType::NAND => Color::MAROON,
            GateType::NOR => Color::CYAN,
            GateType::XNOR => Color::PURPLE,
            GateType::BUF => Color::GREEN,
        };

        sdl.render.color(&color);
        sdl.render.rect(rect);

        if world.get::<&Selected>(entity).is_ok() {
            sdl.render.color(&Color::LIGHTBLUE);
            sdl.render.rect_line(rect);
        }

        const TEXT_SIZE: f32 = 12.0;
        sdl.text.size(TEXT_SIZE);
        sdl.text.color(Color::BLACK);
        sdl.text.render(
            text.0,
            rect.x + TEXT_SIZE / 2.0,
            rect.y + rect.h / 2.0 - TEXT_SIZE / 2.0,
        );

        sdl.render.color(&Color::BLACK);
        const IO_RADIUS: f32 = 4.0;
        for point in &input_points.0 {
            let rect = Rect {
                x: point.x - IO_RADIUS,
                y: point.y - IO_RADIUS,
                w: IO_RADIUS * 2.0,
                h: IO_RADIUS * 2.0,
            };
            sdl.render.rect(&rect);
        }

        let rect = Rect {
            x: rect.x + rect.w - IO_RADIUS,
            y: rect.y + rect.h / 2.0 - IO_RADIUS,
            w: IO_RADIUS * 2.0,
            h: IO_RADIUS * 2.0,
        };
        sdl.render.rect(&rect);
    }
}
