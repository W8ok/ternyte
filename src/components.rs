#[derive(Default, Clone, Debug, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn splat(value: i32) -> Self {
        Self { x: value, y: value }
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, coord: Coordinate) -> bool {
        coord.x >= self.x
            && coord.y >= self.y
            && coord.x < (self.x + self.w)
            && coord.y < (self.y + self.h)
    }
}

pub const GRID_SIZE: i32 = 16;
pub fn snap_to_grid(coord: Coordinate, grid_size: i32) -> Coordinate {
    let x = (coord.x / grid_size) * grid_size;
    let y = (coord.y / grid_size) * grid_size;

    Coordinate::new(x, y)
}
