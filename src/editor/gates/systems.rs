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
    let size = {
        match *kind {
            GateKind::NOT => Coordinate::new(GRID_SIZE * 3, GRID_SIZE * 2),
            GateKind::BUF => Coordinate::new(GRID_SIZE * 3, GRID_SIZE * 2),
            _ => Coordinate::new(GRID_SIZE * 4, GRID_SIZE * 4),
        }
    };

    app.spawn_entity((
        Gate {
            kind: *kind,
            rect: Rect::new(coord.x, coord.y, size.x, size.y),
        },
        PlacingTag,
    ));
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
