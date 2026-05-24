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

    for entity in selected(world) {
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

// Maybe clean this up?
#[allow(static_mut_refs)]
fn move_gates(world: &mut World) {
    struct DragState {
        clicked: bool,
        offsets: Vec<Position>,
    }
    static mut DRAG: DragState = DragState {
        clicked: false,
        offsets: Vec::new(),
    };

    let mouse_pos = input::mouse_pos_camera();

    if input::mouse_pressed(MouseButton::Left) {
        if unsafe { !DRAG.clicked } {
            unsafe {
                DRAG.clicked = true;
                DRAG.offsets.clear();
            }
            for entity in selected(world) {
                let rect = world.get::<&Rect>(entity).unwrap();
                unsafe {
                    DRAG.offsets.push(Position {
                        x: rect.x - mouse_pos.x,
                        y: rect.y - mouse_pos.y,
                    })
                };
            }
        } else {
            let selected_entities = selected(world);
            for (i, entity) in selected_entities.iter().enumerate() {
                if i < unsafe { DRAG.offsets.len() } {
                    let new_pos = Position {
                        x: mouse_pos.x + unsafe { DRAG.offsets[i].x },
                        y: mouse_pos.y + unsafe { DRAG.offsets[i].y },
                    };
                    let snapped = snap_to_grid(new_pos);

                    let mut rect = world.get::<&mut Rect>(*entity).unwrap();
                    rect.x = snapped.x;
                    rect.y = snapped.y;

                    let input_count = world.get::<&InputPoints>(*entity).unwrap().0.len();
                    let mut new_points = Vec::new();
                    for j in 0..input_count {
                        new_points.push(Position {
                            x: snapped.x,
                            y: snapped.y + GRID_SIZE + GRID_SIZE * j as f32,
                        });
                    }
                    *world.get::<&mut InputPoints>(*entity).unwrap() = InputPoints(new_points);
                }
            }
        }
    } else if unsafe { DRAG.clicked } {
        unsafe {
            DRAG.clicked = false;
            DRAG.offsets.clear();
        }
    }
}

fn select_rect(world: &mut World) {
    struct DragState {
        clicked: bool,
        start: Position,
    }
    static mut DRAG: DragState = DragState {
        clicked: false,
        start: Position { x: 0.0, y: 0.0 },
    };

    if input::mouse_pressed(MouseButton::Left) {
        let current_pos = input::mouse_pos_camera();

        if unsafe { !DRAG.clicked } {
            unsafe {
                DRAG.start = current_pos;
                DRAG.clicked = true;
            }
            world.spawn((SelectRect {
                rect: Rect {
                    x: current_pos.x,
                    y: current_pos.y,
                    w: 0.0,
                    h: 0.0,
                },
            },));
        } else {
            let start = unsafe { DRAG.start };
            let mut select_rect = world
                .query_mut::<&mut SelectRect>()
                .into_iter()
                .next()
                .unwrap();

            select_rect.rect = Rect {
                x: start.x.min(current_pos.x),
                y: start.y.min(current_pos.y),
                w: (current_pos.x - start.x).abs(),
                h: (current_pos.y - start.y).abs(),
            };
        }
    } else if unsafe { DRAG.clicked } {
        unsafe { DRAG.clicked = false };

        let select_rect_entity = world
            .query::<Entity>()
            .with::<&SelectRect>()
            .iter()
            .next()
            .unwrap();
        let select_rect = *world.get::<&SelectRect>(select_rect_entity).unwrap();

        let mut to_select = Vec::new();
        for (entity, rect) in world.query::<(Entity, &Rect)>().without::<&Ui>().iter() {
            if select_rect.rect.contains_rect(rect) {
                to_select.push(entity);
            }
        }

        for entity in to_select {
            world.insert_one(entity, Selected).unwrap();
        }

        world.despawn(select_rect_entity);
    }
}

#[inline]
fn selected(world: &World) -> Vec<Entity> {
    world.query::<Entity>().with::<&Selected>().iter().collect()
}

pub fn select_gate(world: &mut World) {
    if input::key_pressed(Key::Escape) || input::mouse_pressed(MouseButton::Right) {
        for entity in selected(world) {
            world.remove_one::<Selected>(entity).unwrap();
        }
    } else if (input::key_pressed(Key::X) && input::key_pressed(Key::Control))
        || input::key_pressed(Key::Delete)
    {
        for entity in selected(world) {
            world.despawn(entity);
        }
    }

    static mut MOVE: bool = false;
    static mut CLICKED: bool = false;
    if input::mouse_pressed(MouseButton::Left) {
        if unsafe { !CLICKED } {
            unsafe { CLICKED = true };

            let pos = input::mouse_pos_camera();

            let mut clicked_entities = Vec::new();
            for (entity, rect) in world.query::<(Entity, &Rect)>().without::<&Ui>().iter() {
                if rect.contains(pos.x, pos.y) {
                    clicked_entities.push(entity);
                }
            }

            if !clicked_entities.is_empty() {
                unsafe { MOVE = true };

                for entity in clicked_entities {
                    if world.get::<&Selected>(entity).is_err() {
                        world.insert_one(entity, Selected).unwrap();
                    }
                }
            } else {
                unsafe { MOVE = false };
            }
        }
    } else {
        unsafe { CLICKED = false };
    }

    if unsafe { MOVE } {
        move_gates(world);
    } else {
        select_rect(world);
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    for (entity, select_rect) in world.query::<(Entity, &SelectRect)>().iter() {
        sdl.render.color(&Color::GREEN);
        sdl.render.rect_line(&select_rect.rect);
    }

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
