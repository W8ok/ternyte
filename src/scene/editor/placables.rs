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

pub fn add_gate(gate_type: GateType, input_count: u8, world: &mut World) {
    let pos = snap_to_grid(input::mouse_pos_camera());
    let rect = Rect {
        x: pos.x,
        y: pos.y,
        w: 64.,
        h: 16. + 16. * input_count as f32,
    };
    world.spawn((rect, gate_type));
}

pub fn place_gate(world: &mut World) {
    let pos = snap_to_grid(input::mouse_pos_camera());
    let to_place: Vec<(Entity, Rect)> = world
        .query::<(Entity, &Rect)>()
        .with::<&GateType>()
        .without::<&Placed>()
        .into_iter()
        .map(|(entity, rect)| {
            // Rustfmt wtf (╥‸╥)
            (
                entity,
                Rect {
                    x: pos.x,
                    y: pos.y,
                    ..*rect
                },
            )
        })
        .collect();

    for (entity, rect) in to_place {
        *world.get::<&mut Rect>(entity).unwrap() = rect;
        if input::mouse_pressed(MouseButton::Left) {
            world.insert_one(entity, Placed).unwrap();
        }
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    for (rect, gate_type) in world.query::<(&Rect, &GateType)>().iter() {
        let color = match gate_type {
            GateType::AND => Color::RED,
            GateType::OR => Color::BLUE,
            GateType::XOR => Color::MAGENTA,
            GateType::NOT => Color::YELLOW,
            GateType::NAND => Color::CYAN,
            GateType::NOR => Color::MAROON,
            GateType::XNOR => Color::ORANGE,
            GateType::BUF => Color::GREEN,
        };

        sdl.render.color(&color);
        sdl.render.rect(rect);
    }
}
