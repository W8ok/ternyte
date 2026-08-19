use glam::Vec2;
use rel_core::*;

use super::super::components::*;
use crate::components::*;
use crate::editor::components::*;
use crate::input::*;
use crate::render::*;

pub fn spawn(app: &mut App) {
    let window_size = get_screen_size();

    app.spawn_entity((Panel {
        rect: Rect::new(0, 0, window_size.x / 5, window_size.y),
        color: Color::DARKGRAY,
        border_color: Color::BLACK,
    },));

    let buttons = [
        "AND", "OR", "NOT", "XOR", "NAND", "NOR", "BUF", "XNOR", "Menu",
    ];
    let spacing = 5;
    let button_width = window_size.x / 5 - spacing * 2;
    let button_height = window_size.y / 10 - spacing * 2;

    for (i, label) in buttons.iter().enumerate() {
        let button_spacing = (spacing + button_height) * i as i32;

        app.spawn_entity((Button {
            rect: Rect::new(
                spacing,
                spacing + button_spacing,
                button_width,
                button_height,
            ),
            text: label,
            pressed: false,
            hovered: false,
            color: Color::GRAY,
            text_color: Color::BLACK,
            border_color: Color::BLACK,
            hovered_color: Color::LIGHTGRAY,
            pressed_color: Color::WHITE,
        },));
    }

    app.res.insert(Camera {
        zoom: 1.0,
        ..Default::default()
    });
}

pub fn despawn(app: &mut App) {
    let mut to_despawn = Vec::new();
    for entity in app.ecs.query::<Entity>().with::<&Button>().iter() {
        to_despawn.push(entity);
    }

    for entity in app.ecs.query::<Entity>().with::<&Panel>().iter() {
        to_despawn.push(entity);
    }

    for entity in app.ecs.query::<Entity>().with::<&Gate>().iter() {
        to_despawn.push(entity);
    }

    for entity in to_despawn.iter() {
        app.ecs.despawn(*entity);
    }

    app.res.remove::<Camera>();
}

pub fn editor(app: &mut App) {
    if is_window_resized() {
        despawn(app);
        spawn(app);
    }

    let window_size = get_screen_size();
    let camera = app.res.get_mut::<Camera>().expect("Camera needed");
    update_camera(&app.ecs, camera, app.time.delta());

    begin_drawing();
    {
        clear_background(Color::LIGHTGRAY);

        begin_mode2d(camera);
        {
            draw_grid(camera, &window_size, Color::DARKGRAY);
            draw_gates(&app.ecs);
        }
        end_mode2d();

        handle_buttons(app, &window_size);
        draw_fps(10, 10);
    }
    end_drawing();
}

fn draw_gates(ecs: &World) {
    for gate in ecs.query::<&Gate>().iter() {
        let color = match gate.kind {
            GateKind::AND => Color::RED,
            GateKind::NAND => Color::ORANGE,
            GateKind::OR => Color::GREEN,
            GateKind::NOR => Color::LIME,
            GateKind::NOT => Color::BLUE,
            GateKind::BUF => Color::MAGENTA,
            GateKind::XOR => Color::WHITE,
            GateKind::XNOR => Color::GRAY,
        };

        draw_rect(&gate.rect, color);

        let text = match gate.kind {
            GateKind::AND => "AND",
            GateKind::NAND => "NAND",
            GateKind::OR => "OR",
            GateKind::NOR => "NOR",
            GateKind::NOT => "NOT",
            GateKind::BUF => "BUF",
            GateKind::XOR => "XOR",
            GateKind::XNOR => "XNOR",
        };

        // TODO
        // Store the fontsize somwhere... or else this will become a nightmare performance wise
        let mut font_size = gate.rect.h;
        let mut text_width = measure_text(text, font_size);
        while text_width >= gate.rect.w {
            font_size -= 1;
            text_width = measure_text(text, font_size);
        }

        let coord = center_text(text_width, font_size, &gate.rect);
        draw_text(text, coord, font_size, Color::BLACK);
    }
}

fn handle_buttons(app: &mut App, window_size: &Coordinate) {
    for panel in app.ecs.query::<&Panel>().iter() {
        panel.draw();
    }

    let mut pressed_buttons = Vec::new();
    for button in app.ecs.query::<&mut Button>().iter() {
        if button.pressed && is_mouse_released(Mouse::MOUSE_BUTTON_LEFT) {
            pressed_buttons.push(button.text);
        }

        button.hovered = button.rect.contains(get_mouse_coord());
        button.pressed = is_mouse_down(Mouse::MOUSE_BUTTON_LEFT) && button.hovered;
        button.draw();
    }

    use crate::editor::components::GateKind;
    for label in pressed_buttons.iter() {
        match *label {
            "AND" => app.send_event(GateKind::AND),
            "NAND" => app.send_event(GateKind::NAND),
            "OR" => app.send_event(GateKind::OR),
            "NOR" => app.send_event(GateKind::NOR),
            "NOT" => app.send_event(GateKind::NOT),
            "BUF" => app.send_event(GateKind::BUF),
            "XOR" => app.send_event(GateKind::XOR),
            "XNOR" => app.send_event(GateKind::XNOR),
            "Menu" => app.set_state(UiState::Menu),
            _ => todo!("Label '{}' not covered", label),
        }
    }
}

fn update_camera(ecs: &World, camera: &mut Camera, dt: f32) {
    let mut direction = Vec2::ZERO;
    if is_key_down(Key::KEY_A) {
        direction.x -= 1.0;
    }
    if is_key_down(Key::KEY_D) {
        direction.x += 1.0;
    }
    if is_key_down(Key::KEY_W) {
        direction.y -= 1.0;
    }
    if is_key_down(Key::KEY_S) {
        direction.y += 1.0;
    }

    if direction != Vec2::ZERO {
        let move_factor = 500.0 * dt;
        let move_scale = 1.0 / camera.zoom;
        let move_vector = direction.normalize() * move_factor;
        camera.target = move_vector * move_scale;
    }

    if is_mouse_down(Mouse::MOUSE_BUTTON_MIDDLE) {
        let mouse_delta = get_mouse_delta();
        camera.target -= mouse_delta / camera.zoom;
    }

    for panel in ecs.query::<&Panel>().iter() {
        let cannot_zoom = panel.rect.contains(get_mouse_coord());
        if cannot_zoom {
            return;
        }
    }

    let zoom_factor = 50.0 * dt;
    let zoom = get_mouse_wheel_move() * zoom_factor;
    if zoom != 0.0 {
        let mouse_pos = get_mouse_pos();
        let old_zoom = camera.zoom;

        let mouse_offset = mouse_pos - camera.offset;
        let mouse_world = mouse_offset / old_zoom + camera.target;

        let new_zoom = (camera.zoom + zoom).clamp(0.1, 10.0);
        let new_offset = (mouse_pos - camera.offset) / new_zoom;

        camera.target = mouse_world - new_offset;
        camera.zoom = new_zoom;
    }
}

fn draw_grid(camera: &Camera, window_size: &Coordinate, color: Color) {
    let spacing = GRID_SIZE as f32;
    let width = window_size.x as f32 / camera.zoom;
    let height = window_size.y as f32 / camera.zoom;

    let left = camera.target.x;
    let top = camera.target.y;
    let right = left + width;
    let bottom = top + height;

    let snap = |v: f32| (v / spacing).floor() * spacing;

    let mut x = snap(left);
    while x <= right {
        draw_line(&Vec2::new(x, top), &Vec2::new(x, bottom), color);
        x += spacing;
    }

    let mut y = snap(top);
    while y <= bottom {
        draw_line(&Vec2::new(left, y), &Vec2::new(right, y), color);
        y += spacing;
    }
}
