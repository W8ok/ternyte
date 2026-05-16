pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub active: bool,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Self {
            x,
            y,
            zoom,
            active: false,
        }
    }

    pub fn update(&mut self, x: f32, y: f32, zoom: f32) {
        self.x = x;
        self.y = y;
        self.zoom = zoom;
    }

    pub fn start(&mut self) {
        self.active = true;
    }

    pub fn end(&mut self) {
        self.active = false;
    }

    pub fn screen_to_world(&self, x: f32, y: f32) -> (f32, f32) {
        let x = x / self.zoom + self.x;
        let y = y / self.zoom + self.y;

        return (x, y);
    }

    pub fn zoom_towards(&mut self, x: f32, y: f32, delta_zoom: f32) {
        let (x2, y2) = self.screen_to_world(x, y);

        self.zoom += delta_zoom;
        self.zoom = self.zoom.clamp(0.1, 10.0);

        self.x = x2 - x / self.zoom;
        self.y = y2 - y / self.zoom;
    }
}
