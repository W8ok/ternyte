use crate::sdl::types::*;

pub enum ButtonAction {
    Exit,
}

pub struct Button {
    pub rect: Rect,
    pub text: String,
    pub pressed: bool,
    pub color: Color,
    pub action: ButtonAction,
}

impl Button {
    pub fn new(rect: Rect, text: &str, action: ButtonAction) -> Self {
        Self {
            rect,
            text: text.to_string(),
            pressed: false,
            color: Color::DARKGRAY,
            action,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.rect.x
            && x <= self.rect.x + self.rect.w
            && y >= self.rect.y
            && y <= self.rect.y + self.rect.h
    }
}
