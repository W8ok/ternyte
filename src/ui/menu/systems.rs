use glam::Vec2;
use rel_core::*;

use super::super::components::*;
use crate::components::*;
use crate::input::*;
use crate::render::*;

pub fn spawn(app: &mut App) {
    let window_size = get_screen_size();

    let buttons = ["Start", "Settings", "About", "Quit"];
    let spacing = 20;
    let button_width = window_size.x / 2 - spacing * 2;
    let button_height = window_size.y / 5 - spacing * 2;

    let panel_width = button_width + spacing * 2;
    let panel_height = button_height * buttons.len() as i32 + spacing * (buttons.len() as i32 + 1);

    let panel_x = window_size.x / 2 - panel_width / 2;
    let panel_y = window_size.y / 2 - panel_height / 2;

    app.spawn_entity((Panel {
        rect: Rect::new(panel_x, panel_y, panel_width, panel_height),
        color: Color::DARKGRAY,
        border_color: Color::BLACK,
    },));

    for (i, label) in buttons.iter().enumerate() {
        let button_spacing = (spacing + button_height) * i as i32;

        app.spawn_entity((Button {
            rect: Rect::new(
                window_size.x / 2 - button_width / 2,
                panel_y + spacing + button_spacing,
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

    app.set_state(UiState::Menu);
}

pub fn despawn(app: &mut App) {
    let mut to_despawn = Vec::new();
    for entity in app.ecs.query::<Entity>().with::<&Button>().iter() {
        to_despawn.push(entity);
    }

    for entity in app.ecs.query::<Entity>().with::<&Panel>().iter() {
        to_despawn.push(entity);
    }

    for entity in to_despawn.iter() {
        app.ecs.despawn(*entity);
    }
}

pub fn menu(app: &mut App) {
    if is_window_resized() {
        despawn(app);
        spawn(app);
    }

    let window_size = get_screen_size();

    begin_drawing();
    {
        clear_background(Color::LIGHTGRAY);

        handle_buttons(app, &window_size);
    }
    end_drawing();
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

    for label in pressed_buttons.iter() {
        match *label {
            "Start" => app.set_state(UiState::Editor),
            "Settings" => {}
            "About" => open_url("https://github.com/W8ok/ternyte"),
            "Quit" => app.quit(),
            _ => todo!("Label '{}' not covered", label),
        }
    }
}
