use crate::input;
use crate::sdl::{
    Sdl,
    event::{Key, MouseButton},
    types::*,
};

pub mod button;
use button::*;

#[derive(PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

pub struct Ui {
    pub buttons: Vec<Button>,
    theme: Theme,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            theme: Theme::Dark,
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn add_button(&mut self, rect: Rect, text: &str, action: ButtonAction) {
        self.buttons.push(Button::new(rect, text, action));
    }

    fn hover(&mut self) {
        let (x, y) = input::mouse_pos();

        for button in self.buttons.iter_mut() {
            if button.contains(x, y) {
                button.color = Color::GRAY;
            } else {
                button.color = Color::DARKGRAY;
            }
        }
    }

    fn click(&mut self) {
        static mut CLICKED: bool = false;
        if input::mouse_pressed(MouseButton::Left) {
            let (x, y) = input::mouse_pos();
            unsafe { CLICKED = true };

            for button in self.buttons.iter_mut() {
                if button.contains(x, y) {
                    button.color = Color::LIGHTGRAY;
                }
            }
        } else if (unsafe { CLICKED }) {
            for button in self.buttons.iter_mut() {
                let (x, y) = input::mouse_pos();
                unsafe { CLICKED = false };

                if button.contains(x, y) {
                    button.pressed = true;
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.hover();
        self.click();
    }

    pub fn render(&mut self, sdl: &mut Sdl) {
        sdl.text.color(Color::WHITE);

        for button in &self.buttons {
            sdl.render.color(&button.color);
            sdl.render.rect(&button.rect);
            sdl.render.color(&Color::BLACK);
            sdl.render.rect_line(&button.rect);

            const PADDING: f32 = 10.0;
            sdl.text.size(button.rect.h - PADDING * 4.);
            sdl.text.render(
                &button.text,
                button.rect.x + PADDING,
                button.rect.y + PADDING * 2.,
            );
        }
    }
}
