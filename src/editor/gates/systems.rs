use rel_core::*;

use super::super::components::*;
use super::components::*;
use crate::components::*;
use crate::input::*;

pub fn spawn_gate(app: &mut App, kind: &GateKind) {
    let coord = {
        let camera = app.res.get::<crate::render::Camera>().expect("Camera gone");
        let mouse_coord = get_mouse_coord();
        screen_to_world(mouse_coord, camera)
    };

    // TODO
    // Make it based on input count?
    // at least for the height
    // for width its whatever
    // also this may break with the addition of rotation

    let (input_count, output_count) = match kind {
        GateKind::UNKNOWN => (0, 0),
        GateKind::AND(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::NAND(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::OR(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::NOR(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::NOT(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::BUF(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::XOR(c) => (c.inputs.len(), c.outputs.len()),
        GateKind::XNOR(c) => (c.inputs.len(), c.outputs.len()),
    };

    let max_connections = input_count.max(output_count);
    let height = GRID_SIZE * 2 * max_connections as i32;
    let width = 6 * GRID_SIZE;
    let size = Coordinate::new(width, height);
    let rect = Rect::new(coord.x, coord.y, size.x, size.y);

    let mut kind = *kind;
    match &mut kind {
        GateKind::AND(c)
        | GateKind::NAND(c)
        | GateKind::OR(c)
        | GateKind::NOR(c)
        | GateKind::XOR(c)
        | GateKind::XNOR(c) => {
            c.inputs[0] = Coordinate::new(rect.x, rect.y + GRID_SIZE);
            c.inputs[1] = Coordinate::new(rect.x, rect.y + height - GRID_SIZE);
            c.outputs[0] = Coordinate::new(rect.x + width, rect.y + height / 2);
        }
        GateKind::NOT(c) | GateKind::BUF(c) => {
            c.inputs[0] = Coordinate::new(rect.x, rect.y + height / 2);
            c.outputs[0] = Coordinate::new(rect.x + width, rect.y + height / 2);
        }
        GateKind::UNKNOWN => {}
    }

    app.spawn_entity((Gate { kind, rect }, PlacingTag));
}

pub fn place_gates(app: &mut App) {
    let coord = {
        let camera = app.res.get::<crate::render::Camera>().expect("Camera gone");
        let mouse_coord = get_mouse_coord();
        let zoom_coord = Coordinate {
            x: (mouse_coord.x as f32 / camera.zoom) as i32,
            y: (mouse_coord.y as f32 / camera.zoom) as i32,
        };
        screen_to_world(zoom_coord, camera)
    };

    let mut offset = 0;
    let padding = GRID_SIZE;
    let mouse_offset = GRID_SIZE / 2; // Sets the gate to be in the middle of the mouse... idk thought it might be nice

    for gate in app.ecs.query::<&mut Gate>().with::<&PlacingTag>().iter() {
        let offset_coord = Coordinate::new(coord.x - mouse_offset, coord.y + offset - mouse_offset);
        let snapped_coord = snap_to_grid(offset_coord, GRID_SIZE);

        gate.rect.x = snapped_coord.x;
        gate.rect.y = snapped_coord.y;

        offset += gate.rect.h + padding;

        match &mut gate.kind {
            GateKind::AND(c)
            | GateKind::NAND(c)
            | GateKind::OR(c)
            | GateKind::NOR(c)
            | GateKind::XOR(c)
            | GateKind::XNOR(c) => {
                c.inputs[0] = Coordinate::new(gate.rect.x, gate.rect.y + GRID_SIZE);
                c.inputs[1] = Coordinate::new(gate.rect.x, gate.rect.y + gate.rect.h - GRID_SIZE);
                c.outputs[0] =
                    Coordinate::new(gate.rect.x + gate.rect.w, gate.rect.y + gate.rect.h / 2);
            }
            GateKind::NOT(c) | GateKind::BUF(c) => {
                c.inputs[0] = Coordinate::new(gate.rect.x, gate.rect.y + gate.rect.h / 2);
                c.outputs[0] =
                    Coordinate::new(gate.rect.x + gate.rect.w, gate.rect.y + gate.rect.h / 2);
            }
            GateKind::UNKNOWN => {}
        }
    }

    if is_mouse_pressed(Mouse::MOUSE_BUTTON_LEFT) {
        for panel in app.ecs.query::<&crate::ui::Panel>().iter() {
            if panel.rect.contains(get_mouse_coord()) {
                return;
            }
        }

        let to_remove: Vec<_> = app
            .ecs
            .query::<Entity>()
            .with::<(&PlacingTag, &Gate)>()
            .iter()
            .collect();

        for entity in to_remove.iter() {
            app.ecs.remove_one::<PlacingTag>(*entity);
        }
    }
}
