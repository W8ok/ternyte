#[derive(Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Default)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const DARKGRAY: Self = Self {
        r: 63,
        g: 63,
        b: 63,
        a: 255,
    };
    pub const GRAY: Self = Self {
        r: 127,
        g: 127,
        b: 127,
        a: 255,
    };
    pub const LIGHTGRAY: Self = Self {
        r: 190,
        g: 190,
        b: 190,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const CLEAR: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}
