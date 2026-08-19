use crate::components::*;
use crate::render::*;

#[derive(Default, Clone, PartialEq, Hash, Debug)]
pub enum UiState {
    #[default]
    Menu,
    Editor,
}

pub struct Panel {
    pub rect: Rect,
    pub color: Color,
    pub border_color: Color,
}

impl Panel {
    pub fn draw(&self) {
        draw_rect(&self.rect, self.color);
        draw_rect_lines(&self.rect, self.border_color);
    }
}

pub struct Button {
    pub rect: Rect,
    pub text: &'static str,
    pub pressed: bool,
    pub hovered: bool,
    pub color: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub hovered_color: Color,
    pub pressed_color: Color,
}

impl Button {
    pub fn draw(&self) {
        let color = if self.pressed {
            self.pressed_color
        } else if self.hovered {
            self.hovered_color
        } else {
            self.color
        };

        draw_rect(&self.rect, color);
        draw_rect_lines(&self.rect, self.border_color);

        let rect = &self.rect;
        let font_size = (rect.h * 4) / 5; // 80% of the height
        let center = Coordinate::new(rect.x + rect.w / 2, rect.y + rect.h / 2);
        let text_width = measure_text(self.text, font_size);
        let coord = Coordinate::new(center.x - text_width / 2, center.y - font_size / 2);
        draw_text(self.text, coord, font_size, self.text_color);
    }
}
