#[derive(Default, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub fn contains_rect(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
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
    pub const DARKRED: Self = Self {
        r: 139,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const DARKGREEN: Self = Self {
        r: 0,
        g: 100,
        b: 0,
        a: 255,
    };
    pub const DARKBLUE: Self = Self {
        r: 0,
        g: 0,
        b: 139,
        a: 255,
    };

    pub const LIGHTRED: Self = Self {
        r: 255,
        g: 100,
        b: 100,
        a: 255,
    };
    pub const LIGHTGREEN: Self = Self {
        r: 144,
        g: 238,
        b: 144,
        a: 255,
    };
    pub const LIGHTBLUE: Self = Self {
        r: 173,
        g: 216,
        b: 230,
        a: 255,
    };

    pub const ORANGE: Self = Self {
        r: 255,
        g: 165,
        b: 0,
        a: 255,
    };
    pub const PURPLE: Self = Self {
        r: 128,
        g: 0,
        b: 128,
        a: 255,
    };
    pub const PINK: Self = Self {
        r: 255,
        g: 105,
        b: 180,
        a: 255,
    };
    pub const BROWN: Self = Self {
        r: 139,
        g: 69,
        b: 19,
        a: 255,
    };

    pub const GOLD: Self = Self {
        r: 255,
        g: 215,
        b: 0,
        a: 255,
    };
    pub const SILVER: Self = Self {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };
    pub const BRONZE: Self = Self {
        r: 205,
        g: 127,
        b: 50,
        a: 255,
    };

    pub const TEAL: Self = Self {
        r: 0,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const LIME: Self = Self {
        r: 50,
        g: 255,
        b: 50,
        a: 255,
    };
    pub const NAVY: Self = Self {
        r: 0,
        g: 0,
        b: 80,
        a: 255,
    };
    pub const MAROON: Self = Self {
        r: 128,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const OLIVE: Self = Self {
        r: 128,
        g: 128,
        b: 0,
        a: 255,
    };
    pub const CORAL: Self = Self {
        r: 255,
        g: 127,
        b: 80,
        a: 255,
    };
    pub const INDIGO: Self = Self {
        r: 75,
        g: 0,
        b: 130,
        a: 255,
    };
}
