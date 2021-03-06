#[derive(Clone, Copy)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
        }
    }
    pub fn update(&mut self) {
        // TODO: collision
        self.x += self.vx;
        self.y += self.vy;
    }
    pub fn serialize(&self) -> [u8; 16] {
        let x = self.x.to_le_bytes();
        let y = self.y.to_le_bytes();
        let vx = self.vx.to_le_bytes();
        let vy = self.vy.to_le_bytes();
        [
            x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3], vx[0], vx[1], vx[2], vx[3], vy[0],
            vy[1], vy[2], vy[3],
        ]
    }
}
