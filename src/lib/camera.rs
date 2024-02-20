use cgmath::{vec3, Angle, Deg, InnerSpace};

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x_rotation: Deg<f32>,
    pub y_rotation: Deg<f32>,
    pub z_rotation: Deg<f32>,
}

impl Camera {
    pub fn new(x: f32, y: f32, z: f32) -> Camera {
        Camera {
            x,
            y,
            z,
            x_rotation: Deg(180.0),
            y_rotation: Deg(0.0),
            z_rotation: Deg(0.0),
        }
    }

    pub fn handle_key(&mut self, key: u8) {
        match key {
            b'w' => {
                self.z -= 0.01 * self.y_rotation.cos() * self.x_rotation.cos();
                self.x -= 0.01 * self.y_rotation.cos() * self.x_rotation.sin();
                self.y += 0.01 * self.y_rotation.sin();
            }
            b's' => {
                self.z += 0.01 * self.y_rotation.cos() * self.x_rotation.cos();
                self.x += 0.01 * self.y_rotation.cos() * self.x_rotation.sin();
                self.y -= 0.01 * self.y_rotation.sin();
            },
            b'a' => {
                self.z -= 0.01 * (Deg(90.0) + self.x_rotation).cos();
                self.x -= 0.01 * (Deg(90.0) + self.x_rotation).sin();
            },
            b'd' => {
                self.z += 0.01 * (Deg(90.0) + self.x_rotation).cos();
                self.x += 0.01 * (Deg(90.0) + self.x_rotation).sin();
            },
            b'q' => self.z += 1.0,
            b'e' => self.z -= 1.0,
            b'1' => {
                self.x_rotation += if self.x_rotation >= Deg(180.0) {
                    Deg(-360.0)
                } else {
                    Deg(1.0)
                }
            }
            b'2' => {
                self.x_rotation -= if self.x_rotation <= Deg(-180.0) {
                    Deg(-360.0)
                } else {
                    Deg(1.0)
                }
            }
            b'3' => {
                self.y_rotation += if self.y_rotation >= Deg(90.0) {
                    Deg(0.0)
                } else {
                    Deg(1.0)
                }
            }
            b'4' => {
                self.y_rotation -= if self.y_rotation <= Deg(-90.0) {
                    Deg(0.0)
                } else {
                    Deg(1.0)
                }
            }
            b'5' => self.z_rotation -= Deg(1.0),
            b'6' => self.z_rotation += Deg(1.0),
            _ => println!("Unknown key"),
        }
        print!(
            "x: {}, y: {}, z: {}, x_view: {:?}, y_view: {:?} y_sin: {} y_cos: {} x_sin: {} x_cos: {}\r",
            self.x, self.y, self.z, self.x_rotation, self.y_rotation,
            self.y_rotation.sin(), self.y_rotation.cos(), self.x_rotation.sin(), self.x_rotation.cos()
        );
    }

    pub fn set_view(&mut self) -> cgmath::Matrix4<f32> {
        let c3 = self.z_rotation.cos();
        let s3 = self.z_rotation.sin();
        let c2 = self.y_rotation.cos();
        let s2 = self.y_rotation.sin();
        let c1 = self.x_rotation.cos();
        let s1 = self.x_rotation.sin();

        let u = vec3(c1 * c3 + s1 * s2 * s3, c2 * s3, c1 * s2 * s3 - c3 * s1);
        let v = vec3(c3 * s1 * s2 - c1 * s3, c2 * c3, c1 * c3 * s2 + s1 * s3);
        let w = vec3(c2 * s1, -s2, c1 * c2);

        let view_matrix = cgmath::Matrix4::new(
            u.x,
            v.x,
            w.x,
            1.0,
            u.y,
            v.y,
            w.y,
            1.0,
            u.z,
            v.z,
            w.z,
            1.0,
            -u.dot(vec3(self.x, self.y, self.z)),
            -v.dot(vec3(self.x, self.y, self.z)),
            -w.dot(vec3(self.x, self.y, self.z)),
            1.0,
        );
        view_matrix
    }
}
