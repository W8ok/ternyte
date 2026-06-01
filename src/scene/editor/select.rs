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

// Maybe clean this up?
#[allow(static_mut_refs)]
fn move_entities(world: &mut World) {
    struct DragState {
        clicked: bool,
        offsets: Vec<Position>,
    }
    static mut DRAG: DragState = DragState {
        clicked: false,
        offsets: Vec::new(),
    };

    let mouse_pos = input::mouse_pos_camera();

    if input::mouse_down(MouseButton::Left) {
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

    if input::mouse_down(MouseButton::Left) {
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

fn deselect_rect(world: &mut World) {
    struct DragState {
        clicked: bool,
        start: Position,
    }
    static mut DRAG: DragState = DragState {
        clicked: false,
        start: Position { x: 0.0, y: 0.0 },
    };

    if input::mouse_down(MouseButton::Right) {
        let current_pos = input::mouse_pos_camera();

        if unsafe { !DRAG.clicked } {
            unsafe {
                DRAG.start = current_pos;
                DRAG.clicked = true;
            }
            world.spawn((DeselectRect {
                rect: Rect {
                    x: current_pos.x,
                    y: current_pos.y,
                    w: 0.0,
                    h: 0.0,
                },
            },));
        } else {
            let start = unsafe { DRAG.start };
            let mut deselect_rect = world
                .query_mut::<&mut DeselectRect>()
                .into_iter()
                .next()
                .unwrap();

            deselect_rect.rect = Rect {
                x: start.x.min(current_pos.x),
                y: start.y.min(current_pos.y),
                w: (current_pos.x - start.x).abs(),
                h: (current_pos.y - start.y).abs(),
            };
        }
    } else if unsafe { DRAG.clicked } {
        unsafe { DRAG.clicked = false };

        let deselect_rect_entity = world
            .query::<Entity>()
            .with::<&DeselectRect>()
            .iter()
            .next()
            .unwrap();
        let deselect_rect = *world.get::<&DeselectRect>(deselect_rect_entity).unwrap();

        let mut to_deselect = Vec::new();
        for (entity, rect) in world.query::<(Entity, &Rect)>().without::<&Ui>().iter() {
            if deselect_rect.rect.contains_rect(rect) {
                to_deselect.push(entity);
            }
        }

        for entity in to_deselect {
            if world.get::<&Selected>(entity).is_ok() {
                world.remove_one::<Selected>(entity).unwrap();
            }
        }

        world.despawn(deselect_rect_entity);
    }
}

pub fn select_entities(world: &mut World) {
    let mut tool = world
        .query_mut::<&mut Tool>()
        .with::<&Resource>()
        .into_iter()
        .next()
        .unwrap();
    if *tool != Tool::Select {
        return;
    }

    if input::mouse_pressed(MouseButton::Right) {
        let pos = input::mouse_pos_camera();

        let clicked_entity = world
            .query::<(Entity, &Rect)>()
            .without::<&Ui>()
            .iter()
            .find(|(_, rect)| rect.contains(pos.x, pos.y))
            .map(|(entity, _)| entity);

        if let Some(entity) = clicked_entity
            && world.get::<&Selected>(entity).is_ok()
        {
            world.remove_one::<Selected>(entity).unwrap();
            return;
        }
    }

    if (input::key_pressed(Key::X) && input::key_pressed(Key::Control))
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

            let clicked_entity = world
                .query::<(Entity, &Rect)>()
                .without::<&Ui>()
                .iter()
                .find(|(_, rect)| rect.contains(pos.x, pos.y))
                .map(|(entity, _)| entity);

            if let Some(entity) = clicked_entity {
                // Check if Shift is held for multi-select
                if input::key_pressed(Key::Shift) {
                    // Toggle selection
                    if world.get::<&Selected>(entity).is_ok() {
                        world.remove_one::<Selected>(entity).unwrap();
                    } else {
                        world.insert_one(entity, Selected).unwrap();
                    }
                    unsafe { MOVE = false }; // Don't start move on toggle
                } else {
                    // Normal selection - if not already selected, clear others and select this one
                    if world.get::<&Selected>(entity).is_err() {
                        for entity in selected(world) {
                            world.remove_one::<Selected>(entity).unwrap();
                        }
                        world.insert_one(entity, Selected).unwrap();
                    }
                    // Start move regardless of whether it was already selected or not
                    unsafe { MOVE = true };
                }
            } else {
                // Clicked on empty space, clear selection and don't move
                for entity in selected(world) {
                    world.remove_one::<Selected>(entity).unwrap();
                }
                unsafe { MOVE = false };
            }
        }
    } else {
        unsafe { CLICKED = false };
    }

    if unsafe { MOVE } {
        move_entities(world);
    } else {
        select_rect(world);
        deselect_rect(world);
    }
}

pub fn render(sdl: &mut Sdl, world: &mut World) {
    for (entity, select_rect) in world.query::<(Entity, &SelectRect)>().iter() {
        sdl.render.color(&Color::GREEN);
        sdl.render.rect_line(&select_rect.rect);
    }

    for (entity, deselect_rect) in world.query::<(Entity, &DeselectRect)>().iter() {
        sdl.render.color(&Color::RED);
        sdl.render.rect_line(&deselect_rect.rect);
    }
}
