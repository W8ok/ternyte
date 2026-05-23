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

pub const GRID_SIZE: f32 = 16.0;

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
                y: pos.y + 16. + 16. * i as f32,
            });
        }
        *world.get::<&mut InputPoints>(entity).unwrap() = InputPoints(points);

        if input::mouse_pressed(MouseButton::Left) {
            world.insert_one(entity, Placed).unwrap();
        }
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    for (rect, gate_type, input_points, text) in world
        .query::<(&Rect, &GateType, &InputPoints, &Text)>()
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
