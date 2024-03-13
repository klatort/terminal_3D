pub struct boid{
    x: f32,
    y: f32,
    z: f32,
    dx: f32,
    dy: f32,
    dz: f32,
}

impl boid{
    pub fn new(x: f32, y: f32, z: f32) -> boid {
        boid {
            x,
            y,
            z,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
        }
    }
    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }
    pub fn get_position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
    pub fn get_velocity(&self) -> (f32, f32, f32) {
        (self.dx, self.dy, self.dz)
    }
}