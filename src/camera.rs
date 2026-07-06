use crate::brep::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Point3,
    pub target: Point3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point3::new(5.0, 2.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            yaw: -45.0f32.to_radians(),
            pitch: 35.264f32.to_radians(),
            fov: 60.0,
        }
    }
}

impl Camera {
    pub fn view_matrix(&self) -> [f32; 16] {
        let forward = self.forward_vector();
        let right = self.right_vector();
        let up = self.up_vector();

        let mut m = [0.0f32; 16];
        m[0] = right.x;
        m[1] = up.x;
        m[2] = -forward.x;
        m[3] = 0.0;
        m[4] = right.y;
        m[5] = up.y;
        m[6] = -forward.y;
        m[7] = 0.0;
        m[8] = right.z;
        m[9] = up.z;
        m[10] = -forward.z;
        m[11] = 0.0;
        m[12] = -right.dot(self.position);
        m[13] = -up.dot(self.position);
        m[14] = forward.dot(self.position);
        m[15] = 1.0;
        m
    }

    pub fn forward_vector(&self) -> Point3 {
        let direction = Point3::new(
            self.target.x - self.position.x,
            self.target.y - self.position.y,
            self.target.z - self.position.z,
        );
        normalize(direction)
    }

    pub fn right_vector(&self) -> Point3 {
        let forward = self.forward_vector();
        let world_up = Point3::new(0.0, 0.0, 1.0);
        let cross = cross(forward, world_up);
        normalize(cross)
    }

    pub fn up_vector(&self) -> Point3 {
        let forward = self.forward_vector();
        let right = self.right_vector();
        cross(right, forward)
    }
}

fn cross(a: Point3, b: Point3) -> Point3 {
    Point3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

impl Point3 {
    fn dot(self, other: Point3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

fn normalize(v: Point3) -> Point3 {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len == 0.0 {
        Point3::new(0.0, 0.0, 0.0)
    } else {
        Point3::new(v.x / len, v.y / len, v.z / len)
    }
}
