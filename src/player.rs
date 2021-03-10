use std::net::SocketAddr;

#[derive(Clone, Copy)]
pub struct Player {
    pub ip: SocketAddr,
    pub y: f32,
    pub vy: f32,
}

impl Player {
    pub fn new(ip: SocketAddr) -> Self {
        Self {
            ip,
            y: 0.0,
            vy: 0.0,
        }
    }
    pub fn update(&mut self) {
        self.y += self.vy;
    }
    pub fn serialize(&self) -> [u8; 8] {
        let y = self.y.to_le_bytes();
        let vy = self.vy.to_le_bytes();
        [y[0], y[1], y[2], y[3], vy[0], vy[1], vy[2], vy[3]]
    }
}
